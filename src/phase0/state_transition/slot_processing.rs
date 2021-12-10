use crate::phase0::beacon_state::BeaconState;
use crate::phase0::mainnet::Error::TransitionToPreviousSlot;
use crate::phase0::state_transition::epoch_processing::process_epoch;
use crate::phase0::state_transition::{Context, Error};
use crate::primitives::{Bytes32, Slot};
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
