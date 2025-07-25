/// Requests information about the chain state as known by the node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {
    /// The Tendermint software semantic version.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The Tendermint block protocol version.
    #[prost(uint64, tag = "2")]
    pub block_version: u64,
    /// The Tendermint p2p protocol version.
    #[prost(uint64, tag = "3")]
    pub p2p_version: u64,
    /// The ABCI semantic version number.
    #[prost(string, tag = "4")]
    pub abci_version: ::prost::alloc::string::String,
}
impl ::prost::Name for InfoRequest {
    const NAME: &'static str = "InfoRequest";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// Contains information about the chain state as known by the node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    /// Some arbitrary information.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// The application software semantic version.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// The application protocol version.
    #[prost(uint64, tag = "3")]
    pub app_version: u64,
    /// The latest block for which the app has called [`Commit`](super::super::Request::Commit).
    #[prost(uint64, tag = "4")]
    pub last_block_height: u64,
    /// The latest result of [`Commit`](super::super::Request::Commit).
    #[prost(bytes = "vec", tag = "5")]
    pub last_block_app_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for InfoResponse {
    const NAME: &'static str = "InfoResponse";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// The root identity key material for a shard operator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardIdentityKey {
    /// An Ed25519 key.
    #[prost(bytes = "vec", tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ShardIdentityKey {
    const NAME: &'static str = "ShardIdentityKey";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// The key that Tendermint will use to identify a validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusKey {
    /// An Ed25519 key.
    #[prost(bytes = "vec", tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ConsensusKey {
    const NAME: &'static str = "ConsensusKey";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A subkey a shard uses to sign messages sent to the ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardMessageKey {
    /// An Ed25519 key.
    #[prost(bytes = "vec", tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ShardMessageKey {
    const NAME: &'static str = "ShardMessageKey";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// The threshold key share controlled by a shard operator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardKey {
    /// A decaf377 scalar.
    #[prost(bytes = "vec", tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ShardKey {
    const NAME: &'static str = "ShardKey";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A signature over a message sent to the ledger by a shard.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardMessageSignature {
    /// An Ed25519 signature.
    #[prost(bytes = "vec", tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ShardMessageSignature {
    const NAME: &'static str = "ShardMessageSignature";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A description of one of the operators of a threshold key share (shard).
///
/// The `ShardOperator` message doesn't have the threshold key share itself,
/// because the workflow is that the set of operators is going to be configured
/// first, as part of the genesis / chain configuration, and then the shards
/// perform DKG as the chain boots, using the chain as the messaging layer for
/// the DKG messages.  This means there's no interactive setup procedure for the
/// shard operators.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardDescription {
    /// The shard operator's offline identity key material which is the root of their authority.
    #[prost(message, optional, tag = "1")]
    pub identity_key: ::core::option::Option<ShardIdentityKey>,
    /// A subkey used for signing messages sent to the ledger.
    #[prost(message, optional, tag = "2")]
    pub message_key: ::core::option::Option<ShardMessageKey>,
    /// The validator's consensus pubkey for use in Tendermint (ed25519)
    #[prost(message, optional, tag = "3")]
    pub consensus_key: ::core::option::Option<ConsensusKey>,
    /// A label for the shard.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
}
impl ::prost::Name for ShardDescription {
    const NAME: &'static str = "ShardDescription";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A self-authenticating `ShardDescription`, signed with the `ShardIdentityKey`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardOperator {
    #[prost(message, optional, tag = "1")]
    pub description: ::core::option::Option<ShardDescription>,
    #[prost(bytes = "vec", tag = "2")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ShardOperator {
    const NAME: &'static str = "ShardOperator";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// The genesis data describing the set of shard operators who jointly control
/// the Narsil instance.
///
/// The genesis data does not specify the threshold key shares themselves,
/// because these will be computed as the ledger boots up and the shard operators
/// perform the DKG to generate the shared key, described by the `ShardInfo`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisData {
    /// The set of shard operators (implicitly specifying the `n` in `t-of-n`).
    #[prost(message, repeated, tag = "1")]
    pub operators: ::prost::alloc::vec::Vec<ShardOperator>,
    /// The number of shards required to sign a message (the `t` in `t-of-n`).
    #[prost(uint32, tag = "2")]
    pub threshold: u32,
}
impl ::prost::Name for GenesisData {
    const NAME: &'static str = "GenesisData";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// Describes the Penumbra wallet id jointly controlled by the Narsil instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletInfo {
    /// The full viewing key for the shared account.
    ///
    /// In the Penumbra key hierarchy, this is the highest-authority key below
    /// spend authority, and allows deriving all subkeys for all accounts in the
    /// wallet id.  It is replicated across all shards.
    ///
    /// The spend verification key component is the `PK` in the FROST I-D.
    #[prost(message, optional, tag = "1")]
    pub full_viewing_key: ::core::option::Option<
        super::super::super::core::keys::v1alpha1::FullViewingKey,
    >,
    /// Describes the participants in the wallet id.
    #[prost(message, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<ShardInfo>,
}
impl ::prost::Name for WalletInfo {
    const NAME: &'static str = "WalletInfo";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// Describes a single shard of the Narsil instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardInfo {
    /// The index of the shard, used for FROST accounting purposes.
    #[prost(uint32, tag = "1")]
    pub index: u32,
    /// The shard verification key, corresponding to `PK_i` in the FROST I-D.
    #[prost(message, optional, tag = "2")]
    pub shard_verification_key: ::core::option::Option<
        super::super::super::core::keys::v1alpha1::SpendVerificationKey,
    >,
    /// The shard operator's identity key, used to identify the operator of this shard.
    #[prost(message, optional, tag = "3")]
    pub identity_key: ::core::option::Option<ShardIdentityKey>,
}
impl ::prost::Name for ShardInfo {
    const NAME: &'static str = "ShardInfo";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// Transaction authorization requests are identified by the proposed
/// transaction's effect hash.
///
/// This acts as a form of content addressing, providing a number of useful
/// behaviors:
///
/// * Multiple users can request authorization of the same `TransactionPlan`, and
///   the ledger can stack their pre-authorizations until some threshold is met.
/// * Rather than having to hold open a connection, clients can re-request
///   authorization of the same `TransactionPlan` after it has been signed, and the
///   ledger can immediately return the already-existing authorization data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestIndex {
    #[prost(message, optional, tag = "1")]
    pub effect_hash: ::core::option::Option<
        super::super::super::core::transaction::v1alpha1::EffectHash,
    >,
}
impl ::prost::Name for RequestIndex {
    const NAME: &'static str = "RequestIndex";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// Identifies a particular signing ceremony.
///
/// Ceremonies are identified first by request index and then by a sub-index for
/// the ceremony.  This allows failed or timed-out ceremonies to be repeated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CeremonyIndex {
    #[prost(message, optional, tag = "1")]
    pub request_index: ::core::option::Option<RequestIndex>,
    #[prost(uint64, tag = "2")]
    pub ceremony_index: u64,
}
impl ::prost::Name for CeremonyIndex {
    const NAME: &'static str = "CeremonyIndex";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A committee of shards assigned to carry out a particular signing ceremony.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Committee {
    #[prost(message, optional, tag = "1")]
    pub ceremony: ::core::option::Option<CeremonyIndex>,
    #[prost(message, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<ShardInfo>,
}
impl ::prost::Name for Committee {
    const NAME: &'static str = "Committee";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// Records a failed ceremony and the reason why it failed.
///
/// TODO: consider filling these in with structured info about the failure
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CeremonyFailure {
    #[prost(oneof = "ceremony_failure::Failure", tags = "1, 2, 3, 4")]
    pub failure: ::core::option::Option<ceremony_failure::Failure>,
}
/// Nested message and enum types in `CeremonyFailure`.
pub mod ceremony_failure {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Timeout {}
    impl ::prost::Name for Timeout {
        const NAME: &'static str = "Timeout";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyFailure.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BadCommitment {}
    impl ::prost::Name for BadCommitment {
        const NAME: &'static str = "BadCommitment";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyFailure.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BadShare {}
    impl ::prost::Name for BadShare {
        const NAME: &'static str = "BadShare";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyFailure.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Canceled {}
    impl ::prost::Name for Canceled {
        const NAME: &'static str = "Canceled";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyFailure.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Failure {
        #[prost(message, tag = "1")]
        Timeout(Timeout),
        #[prost(message, tag = "2")]
        BadCommitment(BadCommitment),
        #[prost(message, tag = "3")]
        BadShare(BadShare),
        #[prost(message, tag = "4")]
        Canceled(Canceled),
    }
}
impl ::prost::Name for CeremonyFailure {
    const NAME: &'static str = "CeremonyFailure";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// The data recorded on-chain about the current state of a signing ceremony.
///
/// The state machine of a signing ceremony is depicted in the following diagram:
///
/// ```text,
/// ┌───────┐   ┌─────────────┐   ┌─────────────┐   ┌────────┐
/// │Pending│──▶│StartedRound1│──▶│StartedRound2│──▶│Finished│
/// └───────┘   └─────────────┘   └─────────────┘   ├────────┤
///     │              │                 │          │AuthData│
///     │              │                 │          └────────┘
///     │              │                 │
///     │              │                 │          ┌────────┐
///     └──────────────┴─────────────────┴─────────▶│ Failed │
///                                                 └────────┘
/// ```
///
/// The ceremony steps are described in the FROST I-D:
/// <https://www.ietf.org/archive/id/draft-irtf-cfrg-frost-11.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CeremonyState {
    #[prost(oneof = "ceremony_state::State", tags = "1, 2, 3, 4, 5")]
    pub state: ::core::option::Option<ceremony_state::State>,
}
/// Nested message and enum types in `CeremonyState`.
pub mod ceremony_state {
    /// A ceremony that has not yet started.
    ///
    /// For instance, a request could be queued until sufficient pre-authorizations were recorded on the ledger.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pending {}
    impl ::prost::Name for Pending {
        const NAME: &'static str = "Pending";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyState.{}", Self::NAME
            )
        }
    }
    /// A ceremony that has started round 1.
    ///
    /// The committee has been chosen and the ledger is waiting to record round 1 contributions from all committee members.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StartedRound1 {
        /// The committee performing the ceremony.
        #[prost(message, optional, tag = "1")]
        pub committee: ::core::option::Option<super::Committee>,
        /// A list of commitment messages received so far (begins empty).
        #[prost(message, repeated, tag = "2")]
        pub commitments: ::prost::alloc::vec::Vec<super::AuthorizeCommitment>,
    }
    impl ::prost::Name for StartedRound1 {
        const NAME: &'static str = "StartedRound1";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyState.{}", Self::NAME
            )
        }
    }
    /// A ceremony that has started round 2.
    ///
    /// The committee has been chosen, all round 1 commitments have been recorded, and the ledger is waiting to record round 1 contributions from all committee members.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StartedRound2 {
        /// The committee performing the ceremony.
        #[prost(message, optional, tag = "1")]
        pub committee: ::core::option::Option<super::Committee>,
        /// A list of commitment messages received in round 1.
        #[prost(message, repeated, tag = "2")]
        pub commitments: ::prost::alloc::vec::Vec<super::AuthorizeCommitment>,
        /// A list of authorization share messages received so far (begins empty).
        #[prost(message, repeated, tag = "3")]
        pub shares: ::prost::alloc::vec::Vec<super::AuthorizeShare>,
    }
    impl ::prost::Name for StartedRound2 {
        const NAME: &'static str = "StartedRound2";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyState.{}", Self::NAME
            )
        }
    }
    /// A ceremony that has successfully finished.
    ///
    /// The transcript of the ceremony is recorded along with the resulting `AuthorizationData`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Finished {
        /// The committee performing the ceremony.
        #[prost(message, optional, tag = "1")]
        pub committee: ::core::option::Option<super::Committee>,
        /// A list of commitment messages received in round 1.
        #[prost(message, repeated, tag = "2")]
        pub commitments: ::prost::alloc::vec::Vec<super::AuthorizeCommitment>,
        /// A list of authorization share messages received in round 2.
        #[prost(message, repeated, tag = "3")]
        pub shares: ::prost::alloc::vec::Vec<super::AuthorizeShare>,
        /// The authorization data resulting from the ceremony.
        #[prost(message, optional, tag = "4")]
        pub auth_data: ::core::option::Option<
            super::super::super::super::core::transaction::v1alpha1::AuthorizationData,
        >,
    }
    impl ::prost::Name for Finished {
        const NAME: &'static str = "Finished";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyState.{}", Self::NAME
            )
        }
    }
    /// A ceremony that failed.
    ///
    /// The transcript of the ceremony is recorded along with the reason for the failure.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Failed {
        /// The committee performing the ceremony.
        #[prost(message, optional, tag = "1")]
        pub committee: ::core::option::Option<super::Committee>,
        /// A list of commitment messages received in round 1, if any.
        #[prost(message, repeated, tag = "2")]
        pub commitments: ::prost::alloc::vec::Vec<super::AuthorizeCommitment>,
        /// A list of authorization share messages received in round 2, if any.
        #[prost(message, repeated, tag = "3")]
        pub shares: ::prost::alloc::vec::Vec<super::AuthorizeShare>,
        /// A description of the failure.
        #[prost(message, optional, tag = "4")]
        pub failure: ::core::option::Option<super::CeremonyFailure>,
    }
    impl ::prost::Name for Failed {
        const NAME: &'static str = "Failed";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.CeremonyState.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum State {
        #[prost(message, tag = "1")]
        Pending(Pending),
        #[prost(message, tag = "2")]
        StartedRound1(StartedRound1),
        #[prost(message, tag = "3")]
        StartedRound2(StartedRound2),
        #[prost(message, tag = "4")]
        Finished(Finished),
        #[prost(message, tag = "5")]
        Failed(Failed),
    }
}
impl ::prost::Name for CeremonyState {
    const NAME: &'static str = "CeremonyState";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A packet of data sent to the Narsil ledger.
///
/// This structure is what Narsil uses as a Tendermint transaction.  However, we
/// use the word "packet" rather than "transaction" here so that it's always
/// unambiguous whether we're referring to data posted to the Penumbra chain or
/// to a Narsil instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NarsilPacket {
    #[prost(oneof = "narsil_packet::Packet", tags = "1, 2, 3, 1000, 1001")]
    pub packet: ::core::option::Option<narsil_packet::Packet>,
}
/// Nested message and enum types in `NarsilPacket`.
pub mod narsil_packet {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Packet {
        /// An authorization request submitted to the Narsil cluster
        ///
        /// Packet handling:
        ///
        /// * check admission policy (black box / ignore for now)
        /// * index the request
        /// * start 1 or more committees to sign it
        #[prost(message, tag = "1")]
        AuthorizeRequest(
            super::super::super::super::custody::v1alpha1::AuthorizeRequest,
        ),
        /// A shard's round 1 contribution to a signing ceremony
        #[prost(message, tag = "2")]
        AuthorizeCommitment(super::AuthorizeCommitment),
        /// A shard's round 2 contribution to a signing ceremony
        #[prost(message, tag = "3")]
        AuthorizeShare(super::AuthorizeShare),
        /// A shard operator's round 1 contribution to the DKG.
        #[prost(message, tag = "1000")]
        DkgRound1(super::DkgRound1),
        /// A shard operator's round 2 contribution to the DKG.
        #[prost(message, tag = "1001")]
        DkgRound2(super::DkgRound2),
    }
}
impl ::prost::Name for NarsilPacket {
    const NAME: &'static str = "NarsilPacket";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A wrapper around the FROST commitment message, exchanged in round 1 of the
/// signing protocol for a single signature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrostCommitment {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for FrostCommitment {
    const NAME: &'static str = "FrostCommitment";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A wrapper around the FROST signature share, exchanged in round 2 of the
/// signing protocol for a single signature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrostSignatureShare {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for FrostSignatureShare {
    const NAME: &'static str = "FrostSignatureShare";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A Narsil shard's commitment message for a single ceremony, which may perform
/// multiple signatures (one for each spend in the `AuthorizeRequest`'s
/// `TransactionPlan`).
///
/// This bundle of messages is signed with the shard's `ShardMessageKey` to
/// prevent tampering (e.g., reordering of the internal FROST messages, etc).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeCommitment {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<authorize_commitment::Body>,
    #[prost(message, optional, tag = "2")]
    pub signer: ::core::option::Option<ShardMessageKey>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<ShardMessageSignature>,
}
/// Nested message and enum types in `AuthorizeCommitment`.
pub mod authorize_commitment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Body {
        #[prost(message, optional, tag = "1")]
        pub ceremony_index: ::core::option::Option<super::CeremonyIndex>,
        #[prost(message, repeated, tag = "2")]
        pub commitments: ::prost::alloc::vec::Vec<super::FrostCommitment>,
    }
    impl ::prost::Name for Body {
        const NAME: &'static str = "Body";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.AuthorizeCommitment.{}", Self::NAME
            )
        }
    }
}
impl ::prost::Name for AuthorizeCommitment {
    const NAME: &'static str = "AuthorizeCommitment";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A Narsil shard's signature share message for a single ceremony, which may perform
/// multiple signatures (one for each spend in the `AuthorizeRequest`'s
/// `TransactionPlan`).
///
/// This bundle of messages is signed with the shard's `ShardMessageKey` to
/// prevent tampering (e.g., reordering of the internal FROST messages, etc).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeShare {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<authorize_share::Body>,
    #[prost(message, optional, tag = "2")]
    pub signer: ::core::option::Option<ShardMessageKey>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<ShardMessageSignature>,
}
/// Nested message and enum types in `AuthorizeShare`.
pub mod authorize_share {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Body {
        #[prost(message, optional, tag = "1")]
        pub ceremony_index: ::core::option::Option<super::CeremonyIndex>,
        #[prost(message, repeated, tag = "2")]
        pub commitments: ::prost::alloc::vec::Vec<super::FrostCommitment>,
    }
    impl ::prost::Name for Body {
        const NAME: &'static str = "Body";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.AuthorizeShare.{}", Self::NAME
            )
        }
    }
}
impl ::prost::Name for AuthorizeShare {
    const NAME: &'static str = "AuthorizeShare";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A shard operator's round 1 contribution to the DKG ceremony.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgRound1 {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub signer: ::core::option::Option<ShardMessageKey>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<ShardMessageSignature>,
}
impl ::prost::Name for DkgRound1 {
    const NAME: &'static str = "DkgRound1";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// A shard operator's round 2 contribution to the DKG ceremony.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgRound2 {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub signer: ::core::option::Option<ShardMessageKey>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<ShardMessageSignature>,
}
impl ::prost::Name for DkgRound2 {
    const NAME: &'static str = "DkgRound2";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// The data recorded on-chain about the current state of the DKG ceremony.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgState {}
/// Nested message and enum types in `DkgState`.
pub mod dkg_state {
    /// The DKG has started round 1, and the ledger is waiting to record contributions from shard operators.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StartedRound1 {
        /// A list of round 1 messages received so far (begins empty).
        #[prost(message, repeated, tag = "1")]
        pub round_1_messages: ::prost::alloc::vec::Vec<super::DkgRound1>,
    }
    impl ::prost::Name for StartedRound1 {
        const NAME: &'static str = "StartedRound1";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.DkgState.{}", Self::NAME
            )
        }
    }
    /// The DKG has started round 2, and the ledger is waiting to record contributions from shard operators.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StartedRound2 {
        /// A list of messages received during round 1.
        #[prost(message, repeated, tag = "1")]
        pub round_1_messages: ::prost::alloc::vec::Vec<super::DkgRound1>,
        /// A list of round 2 messages received so far (begins empty).
        #[prost(message, repeated, tag = "2")]
        pub round_2_messages: ::prost::alloc::vec::Vec<super::DkgRound2>,
    }
    impl ::prost::Name for StartedRound2 {
        const NAME: &'static str = "StartedRound2";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.DkgState.{}", Self::NAME
            )
        }
    }
    /// The DKG has finished successfully, producing the jointly-controlled `WalletInfo`.
    ///
    /// Unlike the signing ceremony, we don't record a failure case here: if the DKG fails, we abort the entire ledger.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Finished {
        /// A list of messages received during round 1.
        #[prost(message, repeated, tag = "1")]
        pub round_1_messages: ::prost::alloc::vec::Vec<super::DkgRound1>,
        /// A list of messages received during round 2.
        #[prost(message, repeated, tag = "2")]
        pub round_2_messages: ::prost::alloc::vec::Vec<super::DkgRound2>,
        /// The jointly-controlled `WalletInfo` resulting from the DKG.
        #[prost(message, optional, tag = "3")]
        pub wallet_info: ::core::option::Option<super::WalletInfo>,
    }
    impl ::prost::Name for Finished {
        const NAME: &'static str = "Finished";
        const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.narsil.ledger.v1alpha1.DkgState.{}", Self::NAME
            )
        }
    }
}
impl ::prost::Name for DkgState {
    const NAME: &'static str = "DkgState";
    const PACKAGE: &'static str = "penumbra.narsil.ledger.v1alpha1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.narsil.ledger.v1alpha1.{}", Self::NAME)
    }
}
/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod ledger_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Methods for narsil clients to communicate with narsild.
    #[derive(Debug, Clone)]
    pub struct LedgerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LedgerServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LedgerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> LedgerServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            LedgerServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn info(
            &mut self,
            request: impl tonic::IntoRequest<super::InfoRequest>,
        ) -> std::result::Result<tonic::Response<super::InfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.narsil.ledger.v1alpha1.LedgerService/Info",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "penumbra.narsil.ledger.v1alpha1.LedgerService",
                        "Info",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod ledger_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LedgerServiceServer.
    #[async_trait]
    pub trait LedgerService: Send + Sync + 'static {
        async fn info(
            &self,
            request: tonic::Request<super::InfoRequest>,
        ) -> std::result::Result<tonic::Response<super::InfoResponse>, tonic::Status>;
    }
    /// Methods for narsil clients to communicate with narsild.
    #[derive(Debug)]
    pub struct LedgerServiceServer<T: LedgerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LedgerService> LedgerServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LedgerServiceServer<T>
    where
        T: LedgerService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/penumbra.narsil.ledger.v1alpha1.LedgerService/Info" => {
                    #[allow(non_camel_case_types)]
                    struct InfoSvc<T: LedgerService>(pub Arc<T>);
                    impl<
                        T: LedgerService,
                    > tonic::server::UnaryService<super::InfoRequest> for InfoSvc<T> {
                        type Response = super::InfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LedgerService>::info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: LedgerService> Clone for LedgerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: LedgerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LedgerService> tonic::server::NamedService for LedgerServiceServer<T> {
        const NAME: &'static str = "penumbra.narsil.ledger.v1alpha1.LedgerService";
    }
}
