use crate::ApiError;
use ethereum_consensus::{
    altair::networking::MetaData,
    capella::Withdrawal,
    networking::{Enr, Multiaddr, PeerId},
    phase0::{Checkpoint, SignedBeaconBlockHeader, Validator},
    primitives::{
        BlsPublicKey, ChainId, CommitteeIndex, Coordinate, Epoch, ExecutionAddress, Gwei, Hash32,
        Root, Slot, ValidatorIndex, Version,
    },
    serde::try_bytes_from_hex_str,
    Fork,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, str::FromStr};

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
    #[serde(with = "crate::serde::as_str")]
    pub chain_id: ChainId,
    pub address: ExecutionAddress,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositSnapshot {
    pub finalized: Vec<Hash32>,
    pub deposit_root: Hash32,
    #[serde(with = "crate::serde::as_str")]
    pub deposit_count: u64,
    pub execution_block_hash: Hash32,
    #[serde(with = "crate::serde::as_str")]
    pub execution_block_height: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GenesisDetails {
    #[serde(with = "crate::serde::as_str")]
    pub genesis_time: u64,
    pub genesis_validators_root: Root,
    #[serde(with = "crate::serde::as_hex")]
    pub genesis_fork_version: Version,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
            StateId::Root(root) => return write!(f, "{root:?}"),
        };
        write!(f, "{printable}")
    }
}

impl FromStr for StateId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "finalized" => Ok(StateId::Finalized),
            "justified" => Ok(StateId::Justified),
            "head" => Ok(StateId::Head),
            "genesis" => Ok(StateId::Genesis),
            _ => match s.parse::<Slot>() {
                Ok(slot) => Ok(Self::Slot(slot)),
                Err(_) => match try_bytes_from_hex_str(s) {
                    Ok(root_data) => {
                        let root = Root::try_from(root_data.as_ref()).map_err(|err| format!("could not parse state identifier by root from the provided argument {s}: {err}"))?;
                        Ok(Self::Root(root))
                    }
                    Err(err) => {
                        let err = format!("could not parse state identifier by root from the provided argument {s}: {err}");
                        Err(err)
                    }
                },
            },
        }
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
            BlockId::Root(root) => return write!(f, "{root:?}"),
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
        match *self {
            Self::PublicKey(ref pk) => write!(f, "{pk:?}"),
            Self::Index(i) => write!(f, "{i}"),
        }
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
    #[serde(with = "crate::serde::as_str")]
    pub index: ValidatorIndex,
    #[serde(with = "crate::serde::as_str")]
    pub balance: Gwei,
    pub status: ValidatorStatus,
    pub validator: Validator,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceSummary {
    #[serde(with = "crate::serde::as_str")]
    pub index: ValidatorIndex,
    #[serde(with = "crate::serde::as_str")]
    pub balance: Gwei,
}

#[derive(Default)]
pub struct CommitteeFilter {
    pub epoch: Option<Epoch>,
    pub index: Option<CommitteeIndex>,
    pub slot: Option<Slot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Committee(#[serde(with = "crate::serde::seq_of_str")] pub Vec<ValidatorIndex>);

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitteeSummary {
    #[serde(with = "crate::serde::as_str")]
    pub index: CommitteeIndex,
    #[serde(with = "crate::serde::as_str")]
    pub slot: Slot,
    pub validators: Committee,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncCommitteeSummary {
    #[serde(with = "crate::serde::seq_of_str")]
    pub validators: Vec<ValidatorIndex>,
    pub validator_aggregates: Vec<Committee>,
}

#[derive(Serialize, Deserialize)]
pub struct BeaconHeaderSummary {
    pub root: Root,
    pub canonical: bool,
    pub signed_header: SignedBeaconBlockHeader,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BroadcastValidation {
    Gossip,
    Consensus,
    ConsensusAndEquivocation,
}

impl fmt::Display for BroadcastValidation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Self::Gossip => "gossip",
            Self::Consensus => "consensus",
            Self::ConsensusAndEquivocation => "consensus_and_equivocation",
        };
        write!(f, "{printable}")
    }
}

pub trait Topic {
    const NAME: &'static str;

    type Data: serde::de::DeserializeOwned;
}

pub struct PayloadAttributesTopic;

impl Topic for PayloadAttributesTopic {
    const NAME: &'static str = "payload_attributes";

    type Data = VersionedValue<PayloadAttributesEvent>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadAttributesEvent {
    #[serde(with = "crate::serde::as_str")]
    pub proposer_index: ValidatorIndex,
    #[serde(with = "crate::serde::as_str")]
    pub proposal_slot: Slot,
    #[serde(with = "crate::serde::as_str")]
    pub parent_block_number: u64,
    pub parent_block_root: Root,
    pub parent_block_hash: Hash32,
    pub payload_attributes: PayloadAttributes,
}

// NOTE: this merges all versions with "optional" fields for
// data defined in subsequent forks
#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadAttributes {
    #[serde(with = "crate::serde::as_str")]
    timestamp: u64,
    prev_randao: Root,
    suggested_fee_recipient: ExecutionAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    withdrawals: Option<Vec<Withdrawal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_beacon_block_root: Option<Root>,
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
    #[serde(with = "crate::serde::as_str")]
    pub disconnected: usize,
    #[serde(with = "crate::serde::as_str")]
    pub connecting: usize,
    #[serde(with = "crate::serde::as_str")]
    pub connected: usize,
    #[serde(with = "crate::serde::as_str")]
    pub disconnecting: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SyncStatus {
    #[serde(with = "crate::serde::as_str")]
    pub head_slot: Slot,
    #[serde(with = "crate::serde::as_str")]
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
    #[serde(with = "crate::serde::as_str")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::as_str")]
    pub committee_index: CommitteeIndex,
    #[serde(with = "crate::serde::as_str")]
    pub committee_length: usize,
    #[serde(with = "crate::serde::as_str")]
    pub committees_at_slot: usize,
    #[serde(with = "crate::serde::as_str")]
    pub validator_committee_index: usize,
    #[serde(with = "crate::serde::as_str")]
    pub slot: Slot,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposerDuty {
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
    #[serde(with = "crate::serde::as_str")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::as_str")]
    pub slot: Slot,
}

#[derive(Serialize, Deserialize)]
pub struct SyncCommitteeDuty {
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
    #[serde(with = "crate::serde::as_str")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::seq_of_str")]
    pub validator_sync_committee_indices: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct CommitteeDescriptor {
    #[serde(with = "crate::serde::as_str")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::as_str")]
    pub committee_index: CommitteeIndex,
    #[serde(with = "crate::serde::as_str")]
    pub committees_at_slot: usize,
    #[serde(with = "crate::serde::as_str")]
    pub slot: Slot,
    pub is_aggregator: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SyncCommitteeDescriptor {
    #[serde(with = "crate::serde::as_str")]
    pub validator_index: ValidatorIndex,
    #[serde(with = "crate::serde::seq_of_str")]
    pub sync_committee_indices: Vec<usize>,
    #[serde(with = "crate::serde::as_str")]
    pub until_epoch: Epoch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeaconProposerRegistration {
    #[serde(with = "crate::serde::as_str")]
    pub validator_index: ValidatorIndex,
    pub fee_recipient: ExecutionAddress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatorLiveness {
    #[serde(with = "crate::serde::as_str")]
    index: ValidatorIndex,
    is_live: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "T: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Value<T> {
    pub data: T,
    #[serde(flatten)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: serde::Serialize + serde::de::DeserializeOwned")]
pub struct VersionedValue<T> {
    pub version: Fork,
    pub data: T,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok(T),
    Err(ApiError),
}
