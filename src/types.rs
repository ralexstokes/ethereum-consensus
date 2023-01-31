use crate::error::ApiError;
use ethereum_consensus::{
    networking::{Enr, MetaData, Multiaddr, PeerId},
    phase0::mainnet::{Checkpoint, SignedBeaconBlockHeader, Validator},
    primitives::{
        BlsPublicKey, ChainId, CommitteeIndex, Coordinate, Epoch, ExecutionAddress, Gwei, Root,
        Slot, ValidatorIndex, Version,
    },
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Serialize, Deserialize)]
pub struct VersionData {
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct CoordinateWithMetadata {
    #[serde(flatten)]
    pub coordinate: Coordinate,
    #[serde(flatten)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct DepositContract {
    #[serde(with = "crate::serde::as_string")]
    pub chain_id: ChainId,
    pub address: ExecutionAddress,
}

#[derive(Serialize, Deserialize)]
pub struct GenesisDetails {
    #[serde(with = "crate::serde::as_string")]
    pub genesis_time: u64,
    pub genesis_validators_root: Root,
    #[serde(with = "crate::serde::as_hex")]
    pub genesis_fork_version: Version,
}

#[derive(Serialize, Deserialize)]
pub enum StateId {
    Head,
    Genesis,
    Finalized,
    Justified,
    Slot(Slot),
    Root(Root),
}

impl fmt::Display for StateId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            StateId::Finalized => "finalized",
            StateId::Justified => "justified",
            StateId::Head => "head",
            StateId::Genesis => "genesis",
            StateId::Slot(slot) => return write!(f, "{slot}"),
            StateId::Root(root) => return write!(f, "{root}"),
        };
        write!(f, "{printable}")
    }
}

#[derive(Serialize, Deserialize)]
pub struct RootData {
    pub root: Root,
}

#[derive(Serialize, Deserialize)]
pub enum BlockId {
    Head,
    Genesis,
    Finalized,
    Slot(Slot),
    Root(Root),
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            BlockId::Finalized => "finalized",
            BlockId::Head => "head",
            BlockId::Genesis => "genesis",
            BlockId::Slot(slot) => return write!(f, "{slot}"),
            BlockId::Root(root) => return write!(f, "{root}"),
        };
        write!(f, "{printable}")
    }
}

#[derive(Serialize, Deserialize)]
enum ExecutionStatus {
    Default,
    Optimistic,
}

#[derive(Serialize, Deserialize)]
pub struct FinalityCheckpoints {
    pub previous_justified: Checkpoint,
    pub current_justified: Checkpoint,
    pub finalized: Checkpoint,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidatorStatus {
    PendingInitialized,
    PendingQueued,
    ActiveOngoing,
    ActiveExiting,
    ActiveSlashed,
    ExitedUnslashed,
    ExitedSlashed,
    WithdrawalPossible,
    WithdrawalDone,
    Active,
    Pending,
    Exited,
    Withdrawal,
}

impl fmt::Display for ValidatorStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Self::PendingInitialized => "pending_initialized",
            Self::PendingQueued => "pending_queued",
            Self::ActiveOngoing => "active_ongoing",
            Self::ActiveExiting => "active_exiting",
            Self::ActiveSlashed => "active_slashed",
            Self::ExitedUnslashed => "exited_unslashed",
            Self::ExitedSlashed => "exited_slashed",
            Self::WithdrawalPossible => "withdrawal_possible",
            Self::WithdrawalDone => "withdrawal_done",
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Exited => "exited",
            Self::Withdrawal => "withdrawal",
        };
        write!(f, "{printable}")
    }
}

#[derive(Debug)]
pub enum PublicKeyOrIndex {
    PublicKey(BlsPublicKey),
    Index(ValidatorIndex),
}

impl fmt::Display for PublicKeyOrIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Self::PublicKey(ref pk) => pk.to_string(),
            Self::Index(i) => i.to_string(),
        };
        write!(f, "{printable}")
    }
}

impl From<ValidatorIndex> for PublicKeyOrIndex {
    fn from(index: ValidatorIndex) -> Self {
        Self::Index(index)
    }
}

impl From<BlsPublicKey> for PublicKeyOrIndex {
    fn from(public_key: BlsPublicKey) -> Self {
        Self::PublicKey(public_key)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatorSummary {
    #[serde(with = "crate::serde::as_string")]
    pub index: ValidatorIndex,
    #[serde(with = "crate::serde::as_string")]
    pub balance: Gwei,
    pub status: ValidatorStatus,
    pub validator: Validator,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceSummary {
    #[serde(with = "crate::serde::as_string")]
    pub index: ValidatorIndex,
    #[serde(with = "crate::serde::as_string")]
    pub balance: Gwei,
}

#[derive(Default)]
pub struct CommitteeFilter {
    pub epoch: Option<Epoch>,
    pub index: Option<CommitteeIndex>,
    pub slot: Option<Slot>,
}

#[derive(Serialize, Deserialize)]
pub struct CommitteeSummary {
    #[serde(with = "crate::serde::as_string")]
    pub index: CommitteeIndex,
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    #[serde(with = "crate::serde::collection_over_string")]
    pub validators: Vec<ValidatorIndex>,
}

#[derive(Serialize, Deserialize)]
pub struct SyncCommitteeSummary {
    #[serde(with = "crate::serde::collection_over_string")]
    pub validators: Vec<ValidatorIndex>,
    // TODO fix serde here
    pub validator_aggregates: Vec<Vec<ValidatorIndex>>,
}

#[derive(Serialize, Deserialize)]
pub struct BeaconHeaderSummary {
    pub root: Root,
    pub canonical: bool,
    pub signed_header: SignedBeaconBlockHeader,
}

pub enum EventTopic {
    Head,
    Block,
    Attestation,
    VoluntaryExit,
    FinalizedCheckpoint,
    ChainReorg,
    ContributionAndProof,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkIdentity {
    pub peer_id: PeerId,
    pub enr: Enr,
    pub p2p_addresses: Vec<Multiaddr>,
    pub discovery_addresses: Vec<Multiaddr>,
    pub metadata: MetaData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PeerState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}

impl fmt::Display for PeerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Self::Disconnected => "disconnected",
            Self::Connecting => "connecting",
            Self::Connected => "connected",
            Self::Disconnecting => "disconnecting",
        };
        write!(f, "{printable}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionOrientation {
    Inbound,
    Outbound,
}

impl fmt::Display for ConnectionOrientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Self::Inbound => "inbound",
            Self::Outbound => "outbound",
        };
        write!(f, "{printable}")
    }
}

#[derive(Serialize, Deserialize)]
pub struct PeerDescriptor {
    pub state: PeerState,
    pub direction: ConnectionOrientation,
}

#[derive(Serialize, Deserialize)]
pub struct PeerDescription {
    pub peer_id: PeerId,
    pub enr: Enr,
    pub last_seen_p2p_address: Multiaddr,
    pub state: PeerState,
    pub direction: ConnectionOrientation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerSummary {
    #[serde(with = "crate::serde::as_string")]
    pub disconnected: usize,
    #[serde(with = "crate::serde::as_string")]
    pub connecting: usize,
    #[serde(with = "crate::serde::as_string")]
    pub connected: usize,
    #[serde(with = "crate::serde::as_string")]
    pub disconnecting: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SyncStatus {
    #[serde(with = "crate::serde::as_string")]
    pub head_slot: Slot,
    #[serde(with = "crate::serde::as_string")]
    pub sync_distance: usize,
    pub is_syncing: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HealthStatus {
    Ready,
    Syncing,
    NotInitialized,
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct AttestationDuty {
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::as_string")]
    pub committee_index: CommitteeIndex,
    #[serde(with = "crate::serde::as_string")]
    pub committee_length: usize,
    #[serde(with = "crate::serde::as_string")]
    pub committees_at_slot: usize,
    #[serde(with = "crate::serde::as_string")]
    pub validator_committee_index: usize,
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposerDuty {
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
}

#[derive(Serialize, Deserialize)]
pub struct SyncCommitteeDuty {
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::collection_over_string")]
    pub validator_sync_committee_indices: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct CommitteeDescriptor {
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::as_string")]
    pub committee_index: CommitteeIndex,
    #[serde(with = "crate::serde::as_string")]
    pub committees_at_slot: usize,
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    pub is_aggregator: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SyncCommitteeDescriptor {
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::collection_over_string")]
    pub sync_committee_indices: Vec<usize>,
    #[serde(with = "crate::serde::as_string")]
    pub until_epoch: Epoch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeaconProposerRegistration {
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    pub fee_recipient: ExecutionAddress,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct Value<T: Serialize + DeserializeOwned> {
    pub data: T,
    #[serde(flatten)]
    pub meta: HashMap<String, serde_json::Value>,
}

/*
`VersionedValue` captures:
```json
{
    "version": "fork-version",
    "data": { ... },
    < optional additional metadata >,
}

And can be combined with Rust `enum`s to handle polymorphic {de,}serialization.
```
 */
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(bound = "T: serde::Serialize + serde::de::DeserializeOwned")]
pub struct VersionedValue<T: serde::Serialize + serde::de::DeserializeOwned> {
    #[serde(flatten)]
    pub payload: T,
    #[serde(flatten)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
#[serde(untagged)]
pub enum ApiResult<T: Serialize + DeserializeOwned> {
    Ok(T),
    Err(ApiError),
}
