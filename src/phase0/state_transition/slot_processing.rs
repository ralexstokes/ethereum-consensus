use crate::phase0::beacon_state::BeaconState;
use crate::phase0::mainnet::Error::{InvalidEpoch, InvalidStateRoot, TransitionToPreviousSlot};
use crate::phase0::operations::PendingAttestation;
use crate::phase0::state_transition::block_processing::process_block;
use crate::phase0::state_transition::epoch_processing::process_epoch;
use crate::phase0::state_transition::{
    get_block_root, get_current_epoch, get_previous_epoch, verify_block_signature, Context, Error,
    SignedBeaconBlock,
};
use crate::primitives::{Bytes32, Epoch, Slot};
use ssz_rs::prelude::*;

pub fn process_slots<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    slot: Slot,
    context: &Context,
) -> Result<(), Error> {
    if state.slot >= slot {
        return Err(TransitionToPreviousSlot {
            requested: slot,
            current: state.slot,
        });
    }
    while state.slot < slot {
        process_slot(state, context)?;
        if (state.slot + 1) % context.slots_per_epoch == 0 {
            process_epoch(state, context)?;
        }
        state.slot += 1;
    }
    Ok(())
}

pub fn process_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    context: &Context,
) -> Result<(), Error> {
    let merkleization_context = context.for_merkleization();

    let previous_state_root = state.hash_tree_root(merkleization_context)?;

    state.state_roots[(state.slot % context.slots_per_historical_root as u64) as usize] =
        previous_state_root;
    if state.latest_block_header.state_root.as_ref() == Bytes32::default().as_ref() {
        state.latest_block_header.state_root = previous_state_root;
    }
    let previous_block_root = state
        .latest_block_header
        .hash_tree_root(merkleization_context)?;
    state.block_roots[(state.slot % context.slots_per_historical_root as u64) as usize] =
        previous_block_root;
    Ok(())
}

pub fn state_transition<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    signed_block: &SignedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    validate_result: Option<bool>,
    context: &Context,
) -> Result<(), Error> {
    let merkleization_context = context.for_merkleization();
    let validate_result = validate_result.unwrap_or(true);
    let block = &signed_block.message;

    process_slots(state, block.slot, context)?;
    if validate_result {
        verify_block_signature(state, signed_block, context)?;
    }
    process_block(state, block, context)?;
    if validate_result && block.state_root != state.hash_tree_root(merkleization_context)? {
        return Err(InvalidStateRoot);
    }

    Ok(())
}

pub fn get_matching_source_attestations<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
    context: &Context,
) -> Result<Vec<PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>>, Error> {
    let previous_epoch = get_previous_epoch(state, context);
    let current_epoch = get_current_epoch(state, context);

    if previous_epoch != epoch && current_epoch != epoch {
        return Err(InvalidEpoch {
            previous: previous_epoch,
            current: current_epoch,
        });
    }

    if epoch == current_epoch {
        return Ok(state.current_epoch_attestations.to_vec());
    }
    Ok(state.previous_epoch_attestations.to_vec())
}

pub fn get_matching_target_attestations<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
    context: &Context,
) -> Result<Vec<PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>>, Error> {
    let source_attestations = get_matching_source_attestations(state, epoch, context)?;
    let block_root = get_block_root(state, epoch, context)?;
    Ok(source_attestations
        .into_iter()
        .filter(|a| a.data.target.root.as_ref() == block_root.as_ref())
        .collect())
}
