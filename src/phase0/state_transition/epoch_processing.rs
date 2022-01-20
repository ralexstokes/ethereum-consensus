use crate::phase0::beacon_state::{BeaconState, HistoricalBatchAccumulator};
use crate::phase0::operations::PendingAttestation;
use crate::phase0::state_transition::{
    compute_activation_exit_epoch, decrease_balance, get_attesting_indices, get_block_root,
    get_block_root_at_slot, get_current_epoch, get_previous_epoch, get_randao_mix,
    get_total_active_balance, get_total_balance, get_validator_churn_limit, increase_balance,
    initiate_validator_exit, is_active_validator, is_eligible_for_activation,
    is_eligible_for_activation_queue, Context, Error,
};
use crate::phase0::BASE_REWARDS_PER_EPOCH;
use crate::primitives::{Epoch, Gwei, ValidatorIndex, GENESIS_EPOCH};
use integer_sqrt::IntegerSquareRoot;
use ssz_rs::prelude::*;
use std::collections::HashSet;
use std::mem;

pub fn get_matching_source_attestations<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
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
) -> Result<
    &'a List<PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>, PENDING_ATTESTATIONS_BOUND>,
    Error,
> {
    let previous_epoch = get_previous_epoch(state, context);
    let current_epoch = get_current_epoch(state, context);

    if previous_epoch != epoch && current_epoch != epoch {
        return Err(Error::InvalidEpoch {
            requested: epoch,
            previous: previous_epoch,
            current: current_epoch,
        });
    }

    if epoch == current_epoch {
        Ok(&state.current_epoch_attestations)
    } else {
        Ok(&state.previous_epoch_attestations)
    }
}

pub fn get_matching_target_attestations<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
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
) -> Result<impl Iterator<Item = &'a PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>>, Error> {
    let source_attestations = get_matching_source_attestations(state, epoch, context)?;
    let block_root = get_block_root(state, epoch, context)?;
    Ok(source_attestations
        .iter()
        .filter(move |a| a.data.target.root == *block_root))
}

pub fn get_matching_head_attestations<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
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
) -> Result<Vec<&'a PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>>, Error> {
    let matching_target_attestations = get_matching_target_attestations(state, epoch, context)?;
    let mut matching_head_attestations = Vec::new();
    for a in matching_target_attestations {
        if a.data.beacon_block_root == *get_block_root_at_slot(state, a.data.slot)? {
            matching_head_attestations.push(a);
        }
    }
    Ok(matching_head_attestations)
}

pub fn get_unslashed_attesting_indices<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    attestations: impl Iterator<Item = &'a PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>>,
    context: &Context,
) -> Result<HashSet<ValidatorIndex>, Error> {
    let mut output = HashSet::new();
    for a in attestations {
        for index in get_attesting_indices(state, &a.data, &a.aggregation_bits, context)? {
            if !state.validators[index].slashed {
                output.insert(index);
            }
        }
    }
    Ok(output)
}

pub fn process_justification_and_finalization<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    _state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    _context: &Context,
) -> Result<(), Error> {
    Ok(())
}

pub fn process_rewards_and_penalties<
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
    // No rewards are applied at the end of `GENESIS_EPOCH` because rewards are for work done in the previous epoch
    let current_epoch = get_current_epoch(state, context);
    if current_epoch != GENESIS_EPOCH {
        let (rewards, penalties) = get_attestation_deltas(state, context)?;
        for i in 0..state.validators.len() {
            increase_balance(state, i, rewards[i]);
            decrease_balance(state, i, penalties[i]);
        }
    }

    Ok(())
}

pub fn process_registry_updates<
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
) {
    // Process activation eligibility and ejections
    let current_epoch = get_current_epoch(state, context);

    for i in 0..state.validators.len() {
        let validator = &mut state.validators[i];
        if is_eligible_for_activation_queue(validator, context) {
            validator.activation_eligibility_epoch = current_epoch + 1;
        }

        if is_active_validator(validator, current_epoch)
            && validator.effective_balance <= context.ejection_balance
        {
            initiate_validator_exit(state, i, context);
        }
    }

    // Queue validators eligible for activation and not yet dequeued for activation
    let mut activation_queue = state
        .validators
        .iter()
        .enumerate()
        .filter_map(|(index, validator)| {
            if is_eligible_for_activation(state, validator) {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<ValidatorIndex>>();
    // Order by the sequence of activation_eligibility_epoch setting and then index
    activation_queue.sort_by(|&i, &j| {
        let a = &state.validators[i];
        let b = &state.validators[j];
        (a.activation_eligibility_epoch, i).cmp(&(b.activation_eligibility_epoch, j))
    });

    // Dequeued validators for activation up to churn limit
    let activation_exit_epoch = compute_activation_exit_epoch(current_epoch, context);
    for i in activation_queue
        .into_iter()
        .take(get_validator_churn_limit(state, context))
    {
        let validator = &mut state.validators[i];
        validator.activation_epoch = activation_exit_epoch;
    }
}

pub fn process_slashings<
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
    let epoch = get_current_epoch(state, context);
    let total_balance = get_total_active_balance(state, context)?;
    let adjusted_total_slashing_balance = Gwei::min(
        state.slashings.iter().sum::<Gwei>() * context.proportional_slashing_multiplier,
        total_balance,
    );

    for i in 0..state.validators.len() {
        let validator = &state.validators[i];
        if validator.slashed
            && (epoch + context.epochs_per_slashings_vector / 2) == validator.withdrawable_epoch
        {
            let increment = context.effective_balance_increment;
            let penalty_numerator =
                validator.effective_balance / increment * adjusted_total_slashing_balance;
            let penalty = penalty_numerator / total_balance * increment;
            decrease_balance(state, i, penalty);
        }
    }

    Ok(())
}

pub fn process_eth1_data_reset<
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
) {
    let next_epoch = get_current_epoch(state, context) + 1;

    if next_epoch % context.epochs_per_eth1_voting_period == 0 {
        state.eth1_data_votes.clear();
    }
}

pub fn process_effective_balance_updates<
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
) {
    // Update effective balances with hysteresis
    let hysteresis_increment = context.effective_balance_increment / context.hysteresis_quotient;
    let downward_threshold = hysteresis_increment * context.hysteresis_downward_multiplier;
    let upward_threshold = hysteresis_increment * context.hysteresis_upward_multiplier;
    for i in 0..state.validators.len() {
        let validator = &mut state.validators[i];
        let balance = state.balances[i];
        if balance + downward_threshold < validator.effective_balance
            || validator.effective_balance + upward_threshold < balance
        {
            validator.effective_balance = Gwei::min(
                balance - balance % context.effective_balance_increment,
                context.max_effective_balance,
            );
        }
    }
}

pub fn process_slashings_reset<
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
) {
    let next_epoch = get_current_epoch(state, context) + 1;

    let slashings_index = next_epoch % context.epochs_per_slashings_vector;
    state.slashings[slashings_index as usize] = 0;
}

pub fn process_randao_mixes_reset<
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
) {
    let current_epoch = get_current_epoch(state, context);
    let next_epoch = current_epoch + 1;
    let mix_index = next_epoch % context.epochs_per_historical_vector;
    state.randao_mixes[mix_index as usize] = get_randao_mix(state, current_epoch).clone();
}

pub fn process_historical_roots_update<
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
    let next_epoch = get_current_epoch(state, context) + 1;
    let epochs_per_historical_root = context.slots_per_historical_root / context.slots_per_epoch;
    if next_epoch % epochs_per_historical_root == 0 {
        let mut historical_batch = HistoricalBatchAccumulator {
            block_roots_root: state.block_roots.hash_tree_root()?,
            state_roots_root: state.state_roots.hash_tree_root()?,
        };
        state
            .historical_roots
            .push(historical_batch.hash_tree_root()?)
    }
    Ok(())
}

pub fn process_participation_record_updates<
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
) {
    let current_attestations = mem::take(&mut state.current_epoch_attestations);
    state.previous_epoch_attestations = current_attestations;
}

pub fn get_base_reward<
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
    index: ValidatorIndex,
    context: &Context,
) -> Result<Gwei, Error> {
    let total_balance = get_total_active_balance(state, context)?;
    let effective_balance = state.validators[index].effective_balance;
    Ok(effective_balance * context.base_reward_factor
        / total_balance.integer_sqrt()
        / BASE_REWARDS_PER_EPOCH)
}

pub fn get_proposer_reward<
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
    attesting_index: ValidatorIndex,
    context: &Context,
) -> Result<Gwei, Error> {
    Ok(get_base_reward(state, attesting_index, context)? / context.proposer_reward_quotient)
}

pub fn get_finality_delay<
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
    context: &Context,
) -> Epoch {
    get_previous_epoch(state, context) - state.finalized_checkpoint.epoch
}

pub fn is_in_inactivity_leak<
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
    context: &Context,
) -> bool {
    get_finality_delay(state, context) > context.min_epochs_to_inactivity_penalty
}

pub fn get_eligible_validator_indices<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
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
) -> impl Iterator<Item = ValidatorIndex> + 'a {
    let previous_epoch = get_previous_epoch(state, context);
    state
        .validators
        .iter()
        .enumerate()
        .filter_map(move |(i, validator)| {
            if is_active_validator(validator, previous_epoch)
                || (validator.slashed && previous_epoch + 1 < validator.withdrawable_epoch)
            {
                Some(i)
            } else {
                None
            }
        })
}

pub fn get_attestation_component_deltas<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    attestations: impl Iterator<Item = &'a PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>>,
    context: &Context,
) -> Result<(Vec<Gwei>, Vec<Gwei>), Error> {
    // Helper with shared logic for use by get source, target, and head deltas functions
    let validator_count = state.validators.len();
    let mut rewards = vec![0; validator_count];
    let mut penalties = vec![0; validator_count];
    let total_balance = get_total_active_balance(state, context)?;
    let unslashed_attesting_indices =
        get_unslashed_attesting_indices(state, attestations, context)?;
    let attesting_balance = get_total_balance(state, &unslashed_attesting_indices, context)?;
    let increment = context.effective_balance_increment; // Factored out from balance totals to avoid uint64 overflow
    for i in get_eligible_validator_indices(state, context) {
        if unslashed_attesting_indices.contains(&i) {
            if is_in_inactivity_leak(state, context) {
                // Since full base reward will be canceled out by inactivity penalty deltas,
                // optimal participation receives full base reward compensation here.
                rewards[i] += get_base_reward(state, i, context)?;
            } else {
                let reward_numerator =
                    get_base_reward(state, i, context)? * (attesting_balance / increment);
                rewards[i] += reward_numerator / (total_balance / increment);
            }
        } else {
            penalties[i] += get_base_reward(state, i, context)?;
        }
    }
    Ok((rewards, penalties))
}

pub fn get_source_deltas<
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
    context: &Context,
) -> Result<(Vec<Gwei>, Vec<Gwei>), Error> {
    // Return attester micro-rewards/penalties for source-vote for each validator.
    let previous_epoch = get_previous_epoch(state, context);
    let matching_source_attestations =
        get_matching_source_attestations(state, previous_epoch, context)?;
    get_attestation_component_deltas(state, matching_source_attestations.iter(), context)
}

pub fn get_target_deltas<
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
    context: &Context,
) -> Result<(Vec<Gwei>, Vec<Gwei>), Error> {
    // Return attester micro-rewards/penalties for target-vote for each validator.
    let previous_epoch = get_previous_epoch(state, context);
    let matching_target_attestations =
        get_matching_target_attestations(state, previous_epoch, context)?;
    get_attestation_component_deltas(state, matching_target_attestations, context)
}

pub fn get_head_deltas<
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
    context: &Context,
) -> Result<(Vec<Gwei>, Vec<Gwei>), Error> {
    // Return attester micro-rewards/penalties for head-vote for each validator.
    let previous_epoch = get_previous_epoch(state, context);
    let matching_head_attestations =
        get_matching_head_attestations(state, previous_epoch, context)?;
    get_attestation_component_deltas(state, matching_head_attestations.into_iter(), context)
}

pub fn get_inclusion_delay_deltas<
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
    context: &Context,
) -> Result<Vec<Gwei>, Error> {
    // Return proposer and inclusion delay micro-rewards/penalties for each validator.
    let previous_epoch = get_previous_epoch(state, context);
    let validator_count = state.validators.len();
    let mut rewards = vec![0; validator_count];
    let matching_source_attestations =
        get_matching_source_attestations(state, previous_epoch, context)?;
    for i in get_unslashed_attesting_indices(state, matching_source_attestations.iter(), context)? {
        let mut attestations = Vec::new();
        for a in matching_source_attestations.iter() {
            if get_attesting_indices(state, &a.data, &a.aggregation_bits, context)?.contains(&i) {
                attestations.push(a)
            }
        }
        let attestation = attestations
            .iter()
            .min_by(|&a, &b| a.inclusion_delay.cmp(&b.inclusion_delay))
            .expect("at least one attestation in collection");
        rewards[attestation.proposer_index] += get_proposer_reward(state, i, context)?;
        let max_attester_reward =
            get_base_reward(state, i, context)? - get_proposer_reward(state, i, context)?;
        rewards[i] += max_attester_reward / attestation.inclusion_delay;
    }
    // No penalties associated with inclusion delay
    // Note: a slight deviation from the spec -- `penalties` is not provided in the return since it's unused
    Ok(rewards)
}

pub fn get_inactivity_penalty_deltas<
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
    context: &Context,
) -> Result<Vec<Gwei>, Error> {
    // Return inactivity reward/penalty deltas for each validator.
    let previous_epoch = get_previous_epoch(state, context);
    let validator_count = state.validators.len();
    let mut penalties = vec![0; validator_count];
    if is_in_inactivity_leak(state, context) {
        let matching_target_attestations =
            get_matching_target_attestations(state, previous_epoch, context)?;
        let matching_target_attesting_indices =
            get_unslashed_attesting_indices(state, matching_target_attestations, context)?;
        for i in get_eligible_validator_indices(state, context) {
            // If validator is performing optimally this cancels all rewards for a neutral balance
            let base_reward = get_base_reward(state, i, context)?;
            penalties[i] +=
                BASE_REWARDS_PER_EPOCH * base_reward - get_proposer_reward(state, i, context)?;
            if !matching_target_attesting_indices.contains(&i) {
                let effective_balance = state.validators[i].effective_balance;
                penalties[i] += effective_balance * get_finality_delay(state, context)
                    / context.inactivity_penalty_quotient;
            }
        }
    }
    // No rewards associated with inactivity penalties
    // Note: a slight deviation from the spec -- `rewards` is not provided in the return since it's unused
    Ok(penalties)
}

pub fn get_attestation_deltas<
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
    context: &Context,
) -> Result<(Vec<Gwei>, Vec<Gwei>), Error> {
    // Return attestation reward/penalty deltas for each validator.
    let (source_rewards, source_penalties) = get_source_deltas(state, context)?;
    let (target_rewards, target_penalties) = get_target_deltas(state, context)?;
    let (head_rewards, head_penalties) = get_head_deltas(state, context)?;
    let inclusion_delay_rewards = get_inclusion_delay_deltas(state, context)?;
    let inactivity_penalties = get_inactivity_penalty_deltas(state, context)?;

    let validator_count = state.validators.len();
    let mut rewards = vec![0; validator_count];
    for i in 0..validator_count {
        rewards[i] =
            source_rewards[i] + target_rewards[i] + head_rewards[i] + inclusion_delay_rewards[i]
    }

    let mut penalties = vec![0; validator_count];
    for i in 0..validator_count {
        penalties[i] =
            source_penalties[i] + target_penalties[i] + head_penalties[i] + inactivity_penalties[i]
    }
    Ok((rewards, penalties))
}

pub fn process_epoch<
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
    process_justification_and_finalization(state, context)?;
    process_rewards_and_penalties(state, context)?;
    process_registry_updates(state, context);
    process_slashings(state, context)?;
    process_eth1_data_reset(state, context);
    process_effective_balance_updates(state, context);
    process_slashings_reset(state, context);
    process_randao_mixes_reset(state, context);
    process_historical_roots_update(state, context)?;
    process_participation_record_updates(state);
    Ok(())
}
