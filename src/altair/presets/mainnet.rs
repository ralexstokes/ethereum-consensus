use crate::altair::presets::Preset;
use crate::phase0::presets::mainnet::{
    BASE_REWARD_FACTOR, EFFECTIVE_BALANCE_INCREMENT, EPOCHS_PER_ETH1_VOTING_PERIOD,
    EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR, HISTORICAL_ROOTS_LIMIT,
    HYSTERESIS_DOWNWARD_MULTIPLIER, HYSTERESIS_QUOTIENT, HYSTERESIS_UPWARD_MULTIPLIER,
    MAX_ATTESTATIONS, MAX_ATTESTER_SLASHINGS, MAX_COMMITTEES_PER_SLOT, MAX_DEPOSITS,
    MAX_EFFECTIVE_BALANCE, MAX_PROPOSER_SLASHINGS, MAX_SEED_LOOKAHEAD,
    MAX_VALIDATORS_PER_COMMITTEE, MAX_VOLUNTARY_EXITS, MIN_ATTESTATION_INCLUSION_DELAY,
    MIN_DEPOSIT_AMOUNT, MIN_EPOCHS_TO_INACTIVITY_PENALTY, MIN_SEED_LOOKAHEAD,
    PROPOSER_REWARD_QUOTIENT, SHUFFLE_ROUND_COUNT, SLOTS_PER_EPOCH, SLOTS_PER_HISTORICAL_ROOT,
    TARGET_COMMITTEE_SIZE, VALIDATOR_REGISTRY_LIMIT, WHISTLEBLOWER_REWARD_QUOTIENT,
};
use crate::primitives::{Epoch, Slot};

pub const INACTIVITY_PENALTY_QUOTIENT_ALTAIR: u64 = 50331648;
pub const MIN_SLASHING_PENALTY_QUOTIENT_ALTAIR: u64 = 64;
pub const PROPORTIONAL_SLASHING_MULTIPLIER_ALTAIR: u64 = 2;
pub const SYNC_COMMITTEE_SIZE: usize = 512;
pub const EPOCHS_PER_SYNC_COMMITTEE_PERIOD: u64 = 256;

pub const PRESET: Preset = Preset {
    max_committees_per_slot: MAX_COMMITTEES_PER_SLOT,
    target_committee_size: TARGET_COMMITTEE_SIZE,
    max_validators_per_committee: MAX_VALIDATORS_PER_COMMITTEE,
    shuffle_round_count: SHUFFLE_ROUND_COUNT,
    hysteresis_quotient: HYSTERESIS_QUOTIENT,
    hysteresis_downward_multiplier: HYSTERESIS_DOWNWARD_MULTIPLIER,
    hysteresis_upward_multiplier: HYSTERESIS_UPWARD_MULTIPLIER,
    min_deposit_amount: MIN_DEPOSIT_AMOUNT,
    max_effective_balance: MAX_EFFECTIVE_BALANCE,
    effective_balance_increment: EFFECTIVE_BALANCE_INCREMENT,
    min_attestation_inclusion_delay: MIN_ATTESTATION_INCLUSION_DELAY,
    slots_per_epoch: SLOTS_PER_EPOCH,
    min_seed_lookahead: MIN_SEED_LOOKAHEAD,
    max_seed_lookahead: MAX_SEED_LOOKAHEAD,
    min_epochs_to_inactivity_penalty: MIN_EPOCHS_TO_INACTIVITY_PENALTY,
    epochs_per_eth1_voting_period: EPOCHS_PER_ETH1_VOTING_PERIOD,
    slots_per_historical_root: SLOTS_PER_HISTORICAL_ROOT as Slot,
    epochs_per_historical_vector: EPOCHS_PER_HISTORICAL_VECTOR as Epoch,
    epochs_per_slashings_vector: EPOCHS_PER_SLASHINGS_VECTOR as Epoch,
    historical_roots_limit: HISTORICAL_ROOTS_LIMIT,
    validator_registry_limit: VALIDATOR_REGISTRY_LIMIT,
    base_reward_factor: BASE_REWARD_FACTOR,
    whistleblower_reward_quotient: WHISTLEBLOWER_REWARD_QUOTIENT,
    proposer_reward_quotient: PROPOSER_REWARD_QUOTIENT,
    inactivity_penalty_quotient: INACTIVITY_PENALTY_QUOTIENT_ALTAIR,
    min_slashing_penalty_quotient: MIN_SLASHING_PENALTY_QUOTIENT_ALTAIR,
    proportional_slashing_multiplier: PROPORTIONAL_SLASHING_MULTIPLIER_ALTAIR,
    max_proposer_slashings: MAX_PROPOSER_SLASHINGS,
    max_attester_slashings: MAX_ATTESTER_SLASHINGS,
    max_attestations: MAX_ATTESTATIONS,
    max_deposits: MAX_DEPOSITS,
    max_voluntary_exits: MAX_VOLUNTARY_EXITS,
    sync_committee_size: SYNC_COMMITTEE_SIZE,
    epochs_per_sync_committee_period: EPOCHS_PER_SYNC_COMMITTEE_PERIOD,
};
