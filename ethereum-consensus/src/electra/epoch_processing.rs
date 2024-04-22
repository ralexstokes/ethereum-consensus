use crate::electra::{
    compute_activation_exit_epoch, get_current_epoch, initiate_validator_exit, is_active_validator,
    is_eligible_for_activation, is_eligible_for_activation_queue, BeaconState, Context, Result,
};

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
) -> Result<()> {
    // Process activation eligibility and ejections
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

    //  Note: Name changed from `activation_exit_epoch` (Deneb) to `activation_epoch` (Electra) in
    // spec.
    let activation_epoch = compute_activation_exit_epoch(current_epoch, context);
    let mut eligible_val_indices = Vec::new();
    for i in 0..state.validators.len() {
        let validator = &state.validators[i];
        if is_eligible_for_activation(state, validator) {
            eligible_val_indices.push(i);
        }
    }

    for i in eligible_val_indices.iter() {
        let validator = &mut state.validators[*i];
        validator.activation_epoch = activation_epoch;
    }

    Ok(())
}
