use crate::crypto::BLSSignature;
pub use crate::phase0::beacon_block::{BeaconBlockHeader, SignedBeaconBlockHeader};
pub use crate::phase0::operations::{
    Deposit, ProposerSlashing, SignedVoluntaryExit, VoluntaryExit,
};
use crate::phase0::{get_eth1_data_votes_bound, get_pending_attestations_bound};
pub use crate::phase0::{
    AttestationData, Checkpoint, DepositData, DepositMessage, Eth1Data, Fork, ForkData,
    SigningData, Validator,
};
use crate::presets::minimal::{
    EPOCHS_PER_ETH1_VOTING_PERIOD, EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR,
    HISTORICAL_ROOTS_LIMIT, MAX_ATTESTATIONS, MAX_ATTESTER_SLASHINGS, MAX_DEPOSITS,
    MAX_PROPOSER_SLASHINGS, MAX_VALIDATORS_PER_COMMITTEE, MAX_VOLUNTARY_EXITS, SLOTS_PER_EPOCH,
    SLOTS_PER_HISTORICAL_ROOT, VALIDATOR_REGISTRY_LIMIT,
};
use crate::primitives::{Bytes32, Gwei, Root, Slot, ValidatorIndex, JUSTIFICATION_BITS_LENGTH};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct IndexedAttestation {
    pub attesting_indices: List<ValidatorIndex, MAX_VALIDATORS_PER_COMMITTEE>,
    pub data: AttestationData,
    pub signature: BLSSignature,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct PendingAttestation {
    pub aggregation_bits: Bitlist<MAX_VALIDATORS_PER_COMMITTEE>,
    pub data: AttestationData,
    pub inclusion_delay: Slot,
    pub proposer_index: ValidatorIndex,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct HistoricalBatch {
    pub block_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub state_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct AttesterSlashing {
    pub attestation_1: IndexedAttestation,
    pub attestation_2: IndexedAttestation,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct Attestation {
    pub aggregation_bits: Bitlist<MAX_VALIDATORS_PER_COMMITTEE>,
    pub data: AttestationData,
    pub signature: BLSSignature,
}

const ETH1_DATA_VOTES_BOUND: usize =
    get_eth1_data_votes_bound(EPOCHS_PER_ETH1_VOTING_PERIOD, SLOTS_PER_EPOCH);

const PENDING_ATTESTATIONS_BOUND: usize =
    get_pending_attestations_bound(MAX_ATTESTATIONS, SLOTS_PER_EPOCH);

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconState {
    pub genesis_time: u64,
    pub genesis_validators_root: Root,
    pub slot: Slot,
    pub fork: Fork,
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub state_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub historical_roots: List<Root, HISTORICAL_ROOTS_LIMIT>,
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: List<Eth1Data, ETH1_DATA_VOTES_BOUND>,
    pub eth1_deposit_index: u64,
    pub validators: List<Validator, VALIDATOR_REGISTRY_LIMIT>,
    pub balances: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
    pub randao_mixes: Vector<Bytes32, EPOCHS_PER_HISTORICAL_VECTOR>,
    pub slashings: Vector<Gwei, EPOCHS_PER_SLASHINGS_VECTOR>,
    pub previous_epoch_attestations: List<PendingAttestation, PENDING_ATTESTATIONS_BOUND>,
    pub current_epoch_attestations: List<PendingAttestation, PENDING_ATTESTATIONS_BOUND>,
    pub justification_bits: Bitvector<JUSTIFICATION_BITS_LENGTH>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconBlockBody {
    pub randao_reveal: BLSSignature,
    pub eth1_data: Eth1Data,
    pub graffiti: Bytes32,
    pub proposer_slashings: List<ProposerSlashing, MAX_PROPOSER_SLASHINGS>,
    pub attester_slashings: List<AttesterSlashing, MAX_ATTESTER_SLASHINGS>,
    pub attestations: List<Attestation, MAX_ATTESTATIONS>,
    pub deposits: List<Deposit, MAX_DEPOSITS>,
    pub voluntary_exits: List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS>,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconBlock {
    pub slot: Slot,
    pub proposer_index: ValidatorIndex,
    pub parent_root: Root,
    pub state_root: Root,
    pub body: BeaconBlockBody,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SignedBeaconBlock {
    pub message: BeaconBlock,
    pub signature: BLSSignature,
}
