/*
use crate::phase0::beacon_state::BeaconState;
use crate::phase0::state_transition::{
    compute_epoch_at_slot, get_active_validator_indices, get_current_epoch,
    get_total_active_balance, get_validator_churn_limit, Context, Error,
};
use crate::primitives::Slot;

pub type Ether = u64;
pub const ETH_TO_GWEI: u64 = 10u64.pow(9);
pub const SAFETY_DECAY: u64 = 10;

pub fn compute_weak_subjectivity_period<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
    const ETH_TO_GWEI: u64,
    const SAFETY_DECAY: u64,
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
) -> Result<u64, Error> {
    // Returns the weak subjectivity period for the current ``state``
    let mut ws_period = context.min_validator_withdrawability_delay;
    let n = get_active_validator_indices(state, get_current_epoch(state, context)).len() as u64;
    let t_1 = get_total_active_balance(state, context)? / n as u64 / ETH_TO_GWEI;
    let t_2 = context.max_effective_balance / ETH_TO_GWEI;
    let delta_1 = get_validator_churn_limit(state, context) as u64;
    let delta_2 = context.max_deposits as u64 * context.slots_per_epoch;
    let d = SAFETY_DECAY;

    if t_2 * (200 + 3 * d) < t_1 * (200 + 12 * d) {
        let epochs_for_validator_set_churn =
            n * (t_1 * (200 + 12 * d) - t_2 * (200 + 3 * d)) / (600 * delta_1 * (2 * t_1 + t_2));
        let epochs_for_balance_top_ups = n * (200 + 3 * d) / (600 * delta_2);
        ws_period += u64::max(epochs_for_validator_set_churn, epochs_for_balance_top_ups);
    } else {
        ws_period += 3 * n * d * t_1 / (200 * delta_2 * (t_2 - t_1));
    }

    Ok(ws_period)
}

pub fn is_within_weak_subjectivity_period<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
    const ETH_TO_GWEI: u64,
    const SAFETY_DECAY: u64,
>(
    store: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    ws_state: &BeaconState<
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
) -> Result<bool, Error> {
    let ws_period = compute_weak_subjectivity_period::<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
        ETH_TO_GWEI,
        SAFETY_DECAY,
    >(ws_state, context)?;
    let ws_state_epoch = compute_epoch_at_slot(ws_state.slot, context);
    let current_epoch = compute_epoch_at_slot(get_current_slot(store), context);

    Ok(current_epoch <= ws_state_epoch + ws_period)
}

fn get_current_slot<
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
) -> Slot {
    state.slot
}

*/
