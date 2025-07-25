use anyhow::Result;

use penumbra_app::genesis;
use penumbra_storage::Storage;
use tendermint::abci::Event;
use tendermint::v0_37::abci::{
    request, response, ConsensusRequest as Request, ConsensusResponse as Response,
};
use tokio::sync::mpsc;
use tower_actor::Message;
use tracing::Instrument;

use crate::App;

pub struct Consensus {
    queue: mpsc::Receiver<Message<Request, Response, tower::BoxError>>,
    storage: Storage,
    app: App,
}

fn trace_events(events: &[Event]) {
    for event in events {
        let span = tracing::info_span!("event", kind = ?event.kind);
        span.in_scope(|| {
            for attr in &event.attributes {
                tracing::info!(k = ?attr.key, v=?attr.value);
            }
        })
    }
}

impl Consensus {
    pub async fn new(
        storage: Storage,
        queue: mpsc::Receiver<Message<Request, Response, tower::BoxError>>,
    ) -> Result<Self> {
        let app = App::new(storage.latest_snapshot()).await?;

        Ok(Self {
            queue,
            storage,
            app,
        })
    }

    pub async fn run(mut self) -> Result<(), tower::BoxError> {
        while let Some(Message {
            req,
            rsp_sender,
            span,
        }) = self.queue.recv().await
        {
            // The send only fails if the receiver was dropped, which happens
            // if the caller didn't propagate the message back to tendermint
            // for some reason -- but that's not our problem.
            let _ = rsp_sender.send(Ok(match req {
                Request::InitChain(init_chain) => Response::InitChain(
                    self.init_chain(init_chain)
                        .instrument(span)
                        .await
                        .expect("init_chain must succeed"),
                ),
                Request::BeginBlock(begin_block) => Response::BeginBlock(
                    self.begin_block(begin_block)
                        .instrument(span)
                        .await
                        .expect("begin_block must succeed"),
                ),
                Request::DeliverTx(deliver_tx) => {
                    Response::DeliverTx(self.deliver_tx(deliver_tx).instrument(span.clone()).await)
                }
                Request::EndBlock(end_block) => Response::EndBlock(
                    self.end_block(end_block)
                        .instrument(span)
                        .await
                        .expect("end_block must succeed"),
                ),
                Request::Commit => Response::Commit(
                    self.commit()
                        .instrument(span)
                        .await
                        .expect("commit must succeed"),
                ),
                Request::PrepareProposal(proposal) => Response::PrepareProposal(
                    self.prepare_proposal(proposal)
                        .instrument(span)
                        .await
                        .expect("prepare proposal must succeed"),
                ),
                Request::ProcessProposal(proposal) => Response::ProcessProposal(
                    self.process_proposal(proposal)
                        .instrument(span)
                        .await
                        .expect("process proposal must succeed"),
                ),
            }));
        }
        Ok(())
    }

    /// Initializes the chain based on the genesis data.
    ///
    /// The genesis data is provided by tendermint, and is used to initialize
    /// the database.
    async fn init_chain(&mut self, init_chain: request::InitChain) -> Result<response::InitChain> {
        // Note that errors cannot be handled in InitChain, the application must crash.
        let app_state: genesis::AppState = serde_json::from_slice(&init_chain.app_state_bytes)
            .expect("can parse app_state in genesis file");

        self.app.init_chain(&app_state).await;

        // Extract the Tendermint validators from the app state
        //
        // NOTE: we ignore the validators passed to InitChain.validators, and instead expect them
        // to be provided inside the initial app genesis state (`GenesisAppState`). Returning those
        // validators in InitChain::Response tells Tendermint that they are the initial validator
        // set. See https://docs.tendermint.com/master/spec/abci/abci.html#initchain
        let validators = self.app.tendermint_validator_updates();

        let app_hash = match &app_state {
            genesis::AppState::Checkpoint(h) => {
                tracing::info!(?h, "genesis state is a checkpoint");
                // If we're starting from a checkpoint, we just need to forward the app hash
                // back to CometBFT.
                self.storage.latest_snapshot().root_hash().await?
            }
            genesis::AppState::Content(_) => {
                tracing::info!("genesis state is a full configuration");
                // Check that we haven't got a duplicated InitChain message for some reason:
                if self.storage.latest_version() != u64::MAX {
                    anyhow::bail!("database already initialized");
                }
                // Note: App::commit resets internal components, so we don't need to do that ourselves.
                self.app.commit(self.storage.clone()).await
            }
        };

        tracing::info!(
            consensus_params = ?init_chain.consensus_params,
            ?validators,
            app_hash = ?app_hash,
            "finished init_chain"
        );

        Ok(response::InitChain {
            consensus_params: Some(init_chain.consensus_params),
            validators,
            app_hash: app_hash.0.to_vec().try_into()?,
        })
    }

    async fn begin_block(
        &mut self,
        begin_block: request::BeginBlock,
    ) -> Result<response::BeginBlock> {
        // We don't need to print the block height, because it will already be
        // included in the span modeling the abci request handling.
        tracing::info!(time = ?begin_block.header.time, "beginning block");
        let events = self.app.begin_block(&begin_block).await;
        Ok(response::BeginBlock { events })
    }

    async fn deliver_tx(&mut self, deliver_tx: request::DeliverTx) -> response::DeliverTx {
        // Unlike the other messages, DeliverTx is fallible, so
        // inspect the response to report errors.
        let rsp = self.app.deliver_tx_bytes(deliver_tx.tx.as_ref()).await;

        match rsp {
            Ok(events) => {
                trace_events(&events);
                response::DeliverTx {
                    events,
                    ..Default::default()
                }
            }
            Err(e) => {
                tracing::info!(?e, "deliver_tx failed");
                response::DeliverTx {
                    code: 1.into(),
                    // Use the alternate format specifier to include the chain of error causes.
                    log: format!("{e:#}"),
                    ..Default::default()
                }
            }
        }
    }

    async fn end_block(&mut self, end_block: request::EndBlock) -> Result<response::EndBlock> {
        let events = self.app.end_block(&end_block).await;
        trace_events(&events);

        // Set `tm_validator_updates` to the complete set of
        // validators and voting power. This must be the last step performed,
        // after all voting power calculations and validator state transitions have
        // been completed.
        let validator_updates = self.app.tendermint_validator_updates();

        tracing::debug!(
            ?validator_updates,
            "sending validator updates to tendermint"
        );

        Ok(response::EndBlock {
            validator_updates,
            consensus_param_updates: None,
            events,
        })
    }

    async fn commit(&mut self) -> Result<response::Commit> {
        let app_hash = self.app.commit(self.storage.clone()).await;
        tracing::info!(?app_hash, "committed block");

        Ok(response::Commit {
            data: app_hash.0.to_vec().into(),
            retain_height: 0u32.into(),
        })
    }

    /// Ensures that a proposal is suitable for processing. Inspects
    /// the transactions within the proposal, discarding any that would
    /// exceed the maximum number of tx bytes for a proposal, and returns
    /// a propoal with the remainder.
    async fn prepare_proposal(
        &mut self,
        proposal: request::PrepareProposal,
    ) -> Result<response::PrepareProposal> {
        let mut txs: Vec<bytes::Bytes> = Vec::new();
        let num_candidate_txs = proposal.txs.len();
        tracing::debug!(
            "processing PrepareProposal, found {} candidate transactions",
            proposal.txs.len()
        );
        // Ensure that list of transactions doesn't exceed max tx bytes. See spec at
        // https://github.com/cometbft/cometbft/blob/v0.37.2/spec/abci/abci%2B%2B_comet_expected_behavior.md#adapting-existing-applications-that-use-abci
        for tx in proposal.txs {
            if (tx.len() + txs.len()) <= proposal.max_tx_bytes.try_into()? {
                txs.push(tx);
            } else {
                break;
            }
        }
        tracing::debug!(
            "finished processing PrepareProposal, including {}/{} candidate transactions",
            txs.len(),
            num_candidate_txs
        );
        Ok(response::PrepareProposal { txs })
    }

    /// Mark the proposal as accepted. The [prepare_proposal] method ensures
    /// that the number of tx bytes is within the acceptable limit.
    async fn process_proposal(
        &mut self,
        proposal: request::ProcessProposal,
    ) -> Result<response::ProcessProposal> {
        tracing::debug!(?proposal, "processing proposal");
        Ok(response::ProcessProposal::Accept)
    }
}
