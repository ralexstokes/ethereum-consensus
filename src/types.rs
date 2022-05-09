use ethereum_consensus::networking::{Enr, MetaData, Multiaddr, PeerId};
use ethereum_consensus::phase0::mainnet::{Checkpoint, SignedBeaconBlockHeader, Validator};
use ethereum_consensus::primitives::{
    BlsPublicKey, CommitteeIndex, Epoch, ExecutionAddress, Gwei, Root, Slot, ValidatorIndex,
    Version,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub enum BlockId {
    Head,
    Genesis,
    Finalized,
    Slot(Slot),
    Root(Root),
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

#[derive(Serialize, Deserialize)]
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
    // TODO what are these?
    Active,
    Pending,
    Exited,
    Withdrawal,
}

pub enum PubkeyOrIndex {
    Pubkey(BlsPublicKey),
    Index(ValidatorIndex),
}

pub struct ValidatorDescriptor {
    pub pubkey_or_index: PubkeyOrIndex,
    pub status: ValidatorStatus,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum PeerState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}

#[derive(Serialize, Deserialize)]
pub enum ConnectionOrientation {
    Inbound,
    Outbound,
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum HealthStatus {
    Ready,
    Syncing,
    NotInitialized,
}

#[derive(Serialize, Deserialize)]
pub struct AttestationDuty {
    pub pubkey: BlsPublicKey,
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

#[derive(Serialize, Deserialize)]
pub struct ProposerDuty {
    pub pubkey: BlsPublicKey,
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
}

#[derive(Serialize, Deserialize)]
pub struct SyncCommitteeDuty {
    pub pubkey: BlsPublicKey,
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::collection_over_string")]
    pub validator_sync_committee_indices: Vec<usize>,
}

pub struct CommitteeDescriptor {
    pub validator_index: ValidatorIndex,
    pub committee_index: CommitteeIndex,
    pub committees_at_slot: usize,
    pub slot: Slot,
    pub is_aggregator: bool,
}

pub struct SyncCommitteeDescriptor {
    pub validator_index: ValidatorIndex,
    pub sync_committee_indices: Vec<usize>,
    pub until_epoch: Epoch,
}

#[derive(Serialize, Deserialize)]
pub struct BeaconProposerRegistration {
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    pub fee_recipient: ExecutionAddress,
}

#[derive(Serialize, Deserialize)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct ApiResponse<T: Serialize + DeserializeOwned> {
    pub data: T,
}

#[derive(Serialize, Deserialize)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct VersionedApiResponse<T: Serialize + DeserializeOwned> {
    pub version: ConsensusVersion,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsensusVersion {
    Phase0,
    Altair,
    Bellatrix,
}
