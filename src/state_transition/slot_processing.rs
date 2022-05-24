use crate::primitives::Slot;
use crate::state_transition::epoch_processing::process_epoch;
use crate::state_transition::{Context, Error};
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
    const SYNC_COMMITTEE_SIZE: usize,
>(
    context: &mut Context<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
        SYNC_COMMITTEE_SIZE,
    >,
    slot: Slot,
) -> Result<(), Error> {
    if context.slot >= slot {
        return Err(Error::TransitionToPreviousSlot {
            requested: slot,
            current: context.slot,
        });
    }
    while context.slot < slot {
        process_slot(context)?;
        if (context.slot + 1) % context.spec.slots_per_epoch == 0 {
            process_epoch(context)?;
        }
        context.slot += 1;
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
    const SYNC_COMMITTEE_SIZE: usize,
>(
    context: &mut Context<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
        SYNC_COMMITTEE_SIZE,
    >,
) -> Result<(), Error> {
    let previous_state_root = context.state_root()?; //state.hash_tree_root()?;
    let root_index = context.slot % context.spec.slots_per_historical_root as u64;
    context.state_roots[root_index as usize] = previous_state_root;

    if context.latest_block_header.state_root == Node::default() {
        context.latest_block_header.state_root = previous_state_root;
    }

    let previous_block_root = context.latest_block_header.hash_tree_root()?;
    let root_index = context.slot % context.spec.slots_per_historical_root as u64;
    context.block_roots[root_index as usize] = previous_block_root;

    Ok(())
}
