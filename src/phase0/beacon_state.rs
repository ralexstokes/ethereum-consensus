use crate::phase0::validator::Validator;
use crate::phase0::{BeaconBlockHeader, Checkpoint, Eth1Data, Fork, PendingAttestation};
use crate::presets::{
    EPOCHS_PER_ETH1_VOTING_PERIOD, EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR,
    HISTORICAL_ROOTS_LIMIT, MAX_ATTESTATIONS, SLOTS_PER_EPOCH, SLOTS_PER_HISTORICAL_ROOT,
    VALIDATOR_REGISTRY_LIMIT,
};
use crate::primitives::{Bytes32, Gwei, Root, Slot, JUSTIFICATION_BITS_LENGTH};
use ssz_rs::prelude::*;

const fn get_eth1_data_votes_bound() -> usize {
    EPOCHS_PER_ETH1_VOTING_PERIOD * SLOTS_PER_EPOCH
}

const ETH1_DATA_VOTES_BOUND: usize = get_eth1_data_votes_bound();

const fn get_pending_attestations_bound() -> usize {
    MAX_ATTESTATIONS * SLOTS_PER_EPOCH
}

const PENDING_ATTESTATIONS_BOUND: usize = get_pending_attestations_bound();

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconState {
    genesis_time: u64,
    genesis_validators_root: Root,
    slot: Slot,
    fork: Fork,
    latest_block_header: BeaconBlockHeader,
    block_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    state_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    historical_roots: List<Root, HISTORICAL_ROOTS_LIMIT>,
    eth1_data: Eth1Data,
    eth1_data_votes: List<Eth1Data, ETH1_DATA_VOTES_BOUND>,
    eth1_deposit_index: u64,
    validators: List<Validator, VALIDATOR_REGISTRY_LIMIT>,
    balances: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
    randao_mixes: Vector<Bytes32, EPOCHS_PER_HISTORICAL_VECTOR>,
    slashings: Vector<Gwei, EPOCHS_PER_SLASHINGS_VECTOR>,
    previous_epoch_attestations: List<PendingAttestation, PENDING_ATTESTATIONS_BOUND>,
    current_epoch_attestations: List<PendingAttestation, PENDING_ATTESTATIONS_BOUND>,
    justification_bits: Bitvector<JUSTIFICATION_BITS_LENGTH>,
    previous_justified_checkpoint: Checkpoint,
    current_justified_checkpoint: Checkpoint,
    finalized_checkpoint: Checkpoint,
}
