use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Context;
use penumbra_compact_block::CompactBlock;
use penumbra_dex::lp::{position, LpNft};
use penumbra_keys::FullViewingKey;
use penumbra_proto::{
    self as proto,
    core::{
        app::v1alpha1::query_service_client::QueryServiceClient as AppQueryServiceClient,
        component::{
            compact_block::v1alpha1::{
                query_service_client::QueryServiceClient as CompactBlockQueryServiceClient,
                CompactBlockRangeRequest,
            },
            shielded_pool::v1alpha1::{
                query_service_client::QueryServiceClient as ShieldedPoolQueryServiceClient,
                DenomMetadataByIdRequest,
            },
        },
    },
};
use penumbra_sct::Nullifier;
use penumbra_transaction::Transaction;
use proto::core::app::v1alpha1::TransactionsByHeightRequest;
use tokio::sync::{watch, RwLock};
use tonic::transport::Channel;
use url::Url;

use crate::{
    sync::{scan_block, FilteredBlock},
    Storage,
};

pub struct Worker {
    storage: Storage,
    sct: Arc<RwLock<penumbra_tct::Tree>>,
    fvk: FullViewingKey, // TODO: notifications (see TODOs on ViewService)
    error_slot: Arc<Mutex<Option<anyhow::Error>>>,
    sync_height_tx: watch::Sender<u64>,
    /// Tonic channel used to create GRPC clients.
    channel: Channel,
    node: Url,
}

impl Worker {
    /// Creates a new worker, returning:
    ///
    /// - the worker itself;
    /// - a shared, in-memory SCT instance;
    /// - a shared error slot;
    /// - a channel for notifying the client of sync progress.
    pub async fn new(
        storage: Storage,
        node: Url,
    ) -> Result<
        (
            Self,
            Arc<RwLock<penumbra_tct::Tree>>,
            Arc<Mutex<Option<anyhow::Error>>>,
            watch::Receiver<u64>,
        ),
        anyhow::Error,
    > {
        let fvk = storage.full_viewing_key().await?;

        // Create a shared, in-memory SCT.
        let sct = Arc::new(RwLock::new(storage.state_commitment_tree().await?));
        // Create a shared error slot
        let error_slot = Arc::new(Mutex::new(None));
        // Create a channel for the worker to notify of sync height changes.
        let (sync_height_tx, mut sync_height_rx) =
            watch::channel(storage.last_sync_height().await?.unwrap_or(0));
        // Mark the current height as seen, since it's not new.
        sync_height_rx.borrow_and_update();

        let channel = Channel::from_shared(node.to_string())
            .with_context(|| "could not parse node URI")?
            .connect()
            .await
            .with_context(|| "could not connect to grpc server")?;

        Ok((
            Self {
                storage,
                sct: sct.clone(),
                fvk,
                error_slot: error_slot.clone(),
                sync_height_tx,
                channel,
                node,
            },
            sct,
            error_slot,
            sync_height_rx,
        ))
    }

    pub async fn fetch_transactions(
        &self,
        filtered_block: &FilteredBlock,
    ) -> anyhow::Result<Vec<Transaction>> {
        let inbound_transaction_ids = filtered_block.inbound_transaction_ids();
        let spent_nullifiers = filtered_block
            .spent_nullifiers
            .iter()
            .cloned()
            .collect::<BTreeSet<Nullifier>>();

        // Only make a block request if we detected transactions in the FilteredBlock.
        // TODO: in the future, we could perform chaff downloads.
        if spent_nullifiers.is_empty() && inbound_transaction_ids.is_empty() {
            return Ok(Vec::new());
        }

        tracing::debug!(
            height = filtered_block.height,
            "fetching full transaction data"
        );

        let transactions = fetch_transactions(self.channel.clone(), filtered_block.height)
            .await?
            .iter()
            .filter_map(|tx| {
                let tx_id = tx.id().0;

                // Check if the transaction is a known inbound transaction or spends one of our nullifiers.
                if inbound_transaction_ids.contains(&tx_id)
                    || tx
                        .spent_nullifiers()
                        .any(|nf| spent_nullifiers.contains(&nf))
                {
                    return Some(tx.clone());
                }
                None
            })
            .collect::<Vec<_>>();

        tracing::debug!(
            matched = transactions.len(),
            "filtered relevant transactions"
        );

        Ok(transactions)
    }

    pub async fn sync(&mut self) -> anyhow::Result<()> {
        // Do a single sync run, up to whatever the latest block height is
        tracing::info!("starting client sync");

        let chain_id = self.storage.app_params().await?.chain_params.chain_id;

        let start_height = self
            .storage
            .last_sync_height()
            .await?
            .map(|h| h + 1)
            .unwrap_or(0);

        let mut client = CompactBlockQueryServiceClient::new(self.channel.clone());
        let mut stream = client
            .compact_block_range(tonic::Request::new(CompactBlockRangeRequest {
                chain_id: chain_id.clone(),
                start_height,
                end_height: 0,
                // Instruct the server to keep feeding us blocks as they're created.
                keep_alive: true,
            }))
            .await?
            .into_inner();

        // Spawn a task to consume items from the stream (somewhat)
        // independently of the execution of the block scanning.  This has two
        // purposes: first, it allows buffering to smooth performance; second,
        // it makes it slightly more difficult for a remote server to observe
        // the exact timings of the scanning of each CompactBlock.
        let (tx, mut buffered_stream) = tokio::sync::mpsc::channel(1000);
        tokio::spawn(async move {
            while let Some(block) = stream.message().await.transpose() {
                if tx.send(block).await.is_err() {
                    break;
                }
            }
        });

        while let Some(block) = buffered_stream.recv().await {
            let block: CompactBlock = block?.try_into()?;

            let height = block.height;

            // Lock the SCT only while processing this block.
            let mut sct_guard = self.sct.write().await;

            if !block.requires_scanning() {
                // Optimization: if the block is empty, seal the in-memory SCT,
                // and skip touching the database:
                sct_guard.end_block()?;
                // We also need to end the epoch, since if there are no funding streams, then an
                // epoch boundary won't necessarily require scanning:
                if block.epoch_root.is_some() {
                    sct_guard
                        .end_epoch()
                        .expect("ending the epoch must succeed");
                }
                self.storage.record_empty_block(height).await?;
                // Notify all watchers of the new height we just recorded.
                self.sync_height_tx.send(height)?;
            } else {
                // Otherwise, scan the block and commit its changes:
                let filtered_block =
                    scan_block(&self.fvk, &mut sct_guard, block, &self.storage).await?;

                // Download any transactions we detected.
                let transactions = self.fetch_transactions(&filtered_block).await?;

                // LPNFT asset IDs won't be known to the chain, so we need to pre-populate them in the local
                // registry based on transaction contents.
                for transaction in &transactions {
                    for action in transaction.actions() {
                        match action {
                            penumbra_transaction::Action::PositionOpen(position_open) => {
                                let position_id = position_open.position.id();

                                // Record every possible permutation.

                                let lp_nft = LpNft::new(position_id, position::State::Opened);
                                let _id = lp_nft.asset_id();
                                let denom = lp_nft.denom();
                                self.storage.record_asset(denom).await?;

                                let lp_nft = LpNft::new(position_id, position::State::Closed);
                                let _id = lp_nft.asset_id();
                                let denom = lp_nft.denom();
                                self.storage.record_asset(denom).await?;

                                let lp_nft = LpNft::new(position_id, position::State::Withdrawn);
                                let _id = lp_nft.asset_id();
                                let denom = lp_nft.denom();
                                self.storage.record_asset(denom).await?;

                                let lp_nft = LpNft::new(position_id, position::State::Claimed);
                                let _id = lp_nft.asset_id();
                                let denom = lp_nft.denom();
                                self.storage.record_asset(denom).await?;

                                // Record the position itself
                                self.storage
                                    .record_position(position_open.position.clone())
                                    .await?;
                            }
                            penumbra_transaction::Action::PositionClose(position_close) => {
                                let position_id = position_close.position_id;

                                // Update the position record
                                self.storage
                                    .update_position(position_id, position::State::Closed)
                                    .await?;
                            }
                            penumbra_transaction::Action::PositionWithdraw(position_withdraw) => {
                                let position_id = position_withdraw.position_id;

                                // Update the position record
                                self.storage
                                    .update_position(position_id, position::State::Withdrawn)
                                    .await?;
                            }
                            penumbra_transaction::Action::PositionRewardClaim(position_claim) => {
                                let position_id = position_claim.position_id;

                                // Update the position record
                                self.storage
                                    .update_position(position_id, position::State::Claimed)
                                    .await?;
                            }
                            _ => (),
                        };
                    }
                }

                // Record any new assets we detected.
                for note_record in &filtered_block.new_notes {
                    // If the asset is already known, skip it.

                    if self
                        .storage
                        .asset_by_id(&note_record.note.asset_id())
                        .await?
                        .is_some()
                    {
                        continue;
                    } else {
                        // If the asset is unknown, we may be able to query for its denom metadata and store that.

                        let mut client = ShieldedPoolQueryServiceClient::new(self.channel.clone());
                        if let Some(denom_metadata) = client
                            .denom_metadata_by_id(DenomMetadataByIdRequest {
                                asset_id: Some(note_record.note.asset_id().into()),
                                chain_id: chain_id.clone(),
                            })
                            .await?
                            .into_inner()
                            .denom_metadata
                        {
                            // If we get metadata: great, record it.
                            self.storage
                                .record_asset(denom_metadata.try_into()?)
                                .await?;
                        } else {
                            // Otherwise we are dealing with an unknown/novel asset ID, but we don't have the original raw denom field naming the asset.
                            // For now, we can just record the asset ID with the denom value as "Unknown".

                            self.storage
                                .record_unknown_asset(note_record.note.asset_id())
                                .await?;
                        }
                    }
                }

                // Commit the block to the database.

                self.storage
                    .record_block(
                        filtered_block.clone(),
                        transactions,
                        &mut sct_guard,
                        self.node.clone(),
                    )
                    .await?;
                // Notify all watchers of the new height we just recorded.
                self.sync_height_tx.send(filtered_block.height)?;
            }
            #[cfg(feature = "sct-divergence-check")]
            sct_divergence_check(self.channel.clone(), height, sct_guard.root()).await?;

            // Release the SCT RwLock
            drop(sct_guard);

            // Check if we should stop waiting for blocks to arrive, because the view
            // services are dropped and we're supposed to shut down.
            if self.sync_height_tx.is_closed() {
                return Ok(());
            }
        }

        Ok(())
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        self.run_inner().await.map_err(|e| {
            tracing::info!(?e, "view worker error");
            self.error_slot
                .lock()
                .expect("no race conditions on worker error slot lock")
                .replace(e);
            anyhow::anyhow!("view worker error")
        })
    }

    async fn run_inner(&mut self) -> anyhow::Result<()> {
        // For now, this can be outside of the loop, because assets are only
        // created at genesis. In the future, we'll want to have a way for
        // clients to learn about assets as they're created.
        self.sync().await?;
        Ok(())
    }
}

// Fetches all transactions in the block.
async fn fetch_transactions(
    channel: Channel,
    block_height: u64,
) -> anyhow::Result<Vec<Transaction>> {
    let mut client = AppQueryServiceClient::new(channel);
    let request = TransactionsByHeightRequest {
        block_height,
        ..Default::default()
    };
    // HACK: this is not a robust long-term solution but may help
    // avoid "split-brain" block fetch issues, where a client learns
    // of a new block, then immediately tries to fetch it, but that
    // fetch is load-balanced over a different node that hasn't yet
    // learned about that block.
    let response = match client.transactions_by_height(request.clone()).await {
        Ok(rsp) => rsp,
        Err(e) => {
            tracing::warn!(?e, "failed to fetch block, waiting and retrying once");
            tokio::time::sleep(Duration::from_secs(1)).await;
            client.transactions_by_height(request).await?
        }
    };
    let transactions = response
        .into_inner()
        .transactions
        .into_iter()
        .map(TryInto::try_into)
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(transactions)
}

#[cfg(feature = "sct-divergence-check")]
async fn sct_divergence_check(
    channel: Channel,
    height: u64,
    actual_root: penumbra_tct::Root,
) -> anyhow::Result<()> {
    use penumbra_proto::{storage::v1alpha1::query_service_client::QueryServiceClient, DomainType};
    use penumbra_sct::state_key as sct_state_key;

    let mut client = QueryServiceClient::new(channel);
    tracing::info!(?height, "fetching anchor @ height");

    let value = client
        .key_value(penumbra_proto::storage::v1alpha1::KeyValueRequest {
            key: sct_state_key::anchor_by_height(height),
            ..Default::default()
        })
        .await?
        .into_inner()
        .value
        .context("sct state not found")?;

    let expected_root = penumbra_tct::Root::decode(value.value.as_slice())?;

    if actual_root == expected_root {
        tracing::info!(?height, ?actual_root, ?expected_root, "sct roots match");
        Ok(())
    } else {
        let e = anyhow::anyhow!(
            "SCT divergence detected at height {}: expected {}, got {}",
            height,
            expected_root,
            actual_root
        );
        // Print the error immediately, so that it's visible in the logs.
        tracing::error!(?e);
        Err(e)
    }
}
