use crate::electra::{
    compute_activation_exit_epoch, get_activation_exit_churn_limit, get_current_epoch,
    increase_balance, initiate_validator_exit, is_active_validator, is_eligible_for_activation,
    is_eligible_for_activation_queue, BeaconState, Context, Error,
};
use ssz_rs::List;

use super::switch_to_compounding_validator;

pub fn process_registry_updates<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const PENDING_BALANCE_DEPOSITS_LIMIT: usize,
    const PENDING_PARTIAL_WITHDRAWALS_LIMIT: usize,
    const PENDING_CONSOLIDATIONS_LIMIT: usize,
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
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        PENDING_BALANCE_DEPOSITS_LIMIT,
        PENDING_PARTIAL_WITHDRAWALS_LIMIT,
        PENDING_CONSOLIDATIONS_LIMIT,
    >,
    context: &Context,
) -> Result<(), Error> {
    let current_epoch = get_current_epoch(state, context);
    for i in 0..state.validators.len() {
        let validator = &mut state.validators[i];
        if is_eligible_for_activation_queue(validator, context) {
            validator.activation_eligibility_epoch = current_epoch + 1;
        }
        if is_active_validator(validator, current_epoch) &&
            validator.effective_balance <= context.ejection_balance
        {
            initiate_validator_exit(state, i, context)?;
        }
    }

    let activation_epoch = compute_activation_exit_epoch(current_epoch, context);
    for i in 0..state.validators.len() {
        let validator = &state.validators[i];
        if is_eligible_for_activation(state, validator) {
            let validator = &mut state.validators[i];
            validator.activation_epoch = activation_epoch;
        }
    }

    Ok(())
}

pub fn process_pending_balance_deposits<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const PENDING_BALANCE_DEPOSITS_LIMIT: usize,
    const PENDING_PARTIAL_WITHDRAWALS_LIMIT: usize,
    const PENDING_CONSOLIDATIONS_LIMIT: usize,
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
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        PENDING_BALANCE_DEPOSITS_LIMIT,
        PENDING_PARTIAL_WITHDRAWALS_LIMIT,
        PENDING_CONSOLIDATIONS_LIMIT,
    >,
    context: &Context,
) -> Result<(), Error> {
    let available_for_processing =
        state.deposit_balance_to_consume + get_activation_exit_churn_limit(state, context)?;
    let mut processed_amount = 0;
    let mut next_deposit_index = 0;

    for i in 0..state.pending_balance_deposits.len() {
        let deposit = &state.pending_balance_deposits[i];
        let index = deposit.index;
        let amount = deposit.amount;
        if processed_amount + deposit.amount > available_for_processing {
            break
        }
        increase_balance(state, index, amount);
        processed_amount += amount;
        next_deposit_index += 1;
    }

    state.pending_balance_deposits =
        List::try_from(state.pending_balance_deposits.split_off(next_deposit_index)).unwrap();

    if state.pending_balance_deposits.len() == 0 {
        state.deposit_balance_to_consume = 0;
    } else {
        state.deposit_balance_to_consume = available_for_processing - processed_amount;
    }

    Ok(())
}

pub fn process_pending_consolidations<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const PENDING_BALANCE_DEPOSITS_LIMIT: usize,
    const PENDING_PARTIAL_WITHDRAWALS_LIMIT: usize,
    const PENDING_CONSOLIDATIONS_LIMIT: usize,
    const MAX_VALIDATORS_PER_SLOT: usize,
    const MAX_COMMITTEES_PER_SLOT: usize,
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
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        PENDING_BALANCE_DEPOSITS_LIMIT,
        PENDING_PARTIAL_WITHDRAWALS_LIMIT,
        PENDING_CONSOLIDATIONS_LIMIT,
    >,
    context: &Context,
) -> Result<(), Error> {
    todo!()
}
