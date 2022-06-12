// TODO remove once impl is added
#![allow(dead_code)]

use crate::altair as spec;

use crate::primitives::{Epoch, Gwei, ParticipationFlags, ValidatorIndex};
use crate::state_transition::{Context, Result};
use spec::{
    decrease_balance, get_beacon_proposer_index, get_current_epoch, get_eligible_validator_indices,
    get_previous_epoch, increase_balance, initiate_validator_exit, BeaconState, PROPOSER_WEIGHT,
    TIMELY_TARGET_FLAG_INDEX, WEIGHT_DENOMINATOR,
};
use std::collections::HashSet;

pub fn add_flag(flags: ParticipationFlags, flag_index: u8) -> ParticipationFlags {
    // Return a new ``ParticipationFlags`` adding ``flag_index`` to ``flags``
    let flag = 2u8.pow(flag_index as u32);
    flags | flag
}

pub fn has_flag(flags: ParticipationFlags, flag_index: u8) -> bool {
    // Return whether ``flags`` has ``flag_index`` set
    let flag = 2u8.pow(flag_index as u32);
    flags & flag == flag
}

pub fn get_next_sync_committee_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    _state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    _context: &Context,
) -> Result<()> {
    Ok(())
}

pub fn get_next_sync_committee<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    _state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    _context: &Context,
) -> Result<()> {
    Ok(())
}

pub fn get_base_reward_per_increment<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    _state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    _context: &Context,
) -> Result<()> {
    Ok(())
}

pub fn get_base_reward<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    _state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    _context: &Context,
) -> Result<()> {
    Ok(())
}

pub fn get_unslashed_participating_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    _state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    _flag_index: usize,
    _epoch: Epoch,
    _context: &Context,
) -> Result<HashSet<ValidatorIndex>> {
    Ok(Default::default())
}

pub fn get_attestation_participation_flag_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    _state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    _context: &Context,
) -> Result<()> {
    Ok(())
}

pub fn get_flag_index_deltas<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    _state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    _context: &Context,
) -> Result<()> {
    Ok(())
}

pub fn get_inactivity_penalty_deltas<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    context: &Context,
) -> Result<(Vec<Gwei>, Vec<Gwei>)> {
    let validator_count = state.validators.len();
    let rewards = vec![0; validator_count];
    let mut penalties = vec![0; validator_count];
    let previous_epoch = get_previous_epoch(state, context);
    let matching_target_indices = get_unslashed_participating_indices(
        state,
        TIMELY_TARGET_FLAG_INDEX,
        previous_epoch,
        context,
    )?;
    let current_epoch = get_current_epoch(state, context);
    let inactivity_penalty_quotient = context.inactivity_penalty_quotient(current_epoch)?;
    for i in get_eligible_validator_indices(state, context) {
        if !matching_target_indices.contains(&i) {
            let penalty_numerator =
                state.validators[i].effective_balance * state.inactivity_scores[i];
            let penalty_denominator = context.inactivity_score_bias * inactivity_penalty_quotient;
            penalties[i] += penalty_numerator / penalty_denominator;
        }
    }
    Ok((rewards, penalties))
}

pub fn slash_validator<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    slashed_index: ValidatorIndex,
    whistleblower_index: Option<ValidatorIndex>,
    context: &Context,
) -> Result<()> {
    let epoch = get_current_epoch(state, context);
    initiate_validator_exit(state, slashed_index, context);
    state.validators[slashed_index].slashed = true;
    state.validators[slashed_index].withdrawable_epoch = u64::max(
        state.validators[slashed_index].withdrawable_epoch,
        epoch + context.epochs_per_slashings_vector as u64,
    );
    let slashings_index = epoch as usize % EPOCHS_PER_SLASHINGS_VECTOR;
    state.slashings[slashings_index] += state.validators[slashed_index].effective_balance;
    let min_slashing_penalty_quotient = context.min_slashing_penalty_quotient(epoch)?;
    decrease_balance(
        state,
        slashed_index,
        state.validators[slashed_index].effective_balance / min_slashing_penalty_quotient,
    );

    let proposer_index = get_beacon_proposer_index(state, context)?;

    let whistleblower_index = whistleblower_index.unwrap_or(proposer_index);

    let whistleblower_reward =
        state.validators[slashed_index].effective_balance / context.whistleblower_reward_quotient;
    let proposer_reward_scaling_factor = PROPOSER_WEIGHT / WEIGHT_DENOMINATOR;
    let proposer_reward = whistleblower_reward * proposer_reward_scaling_factor;
    increase_balance(state, proposer_index, proposer_reward);
    increase_balance(
        state,
        whistleblower_index,
        whistleblower_reward - proposer_reward,
    );
    Ok(())
}
