use crate::altair::configs::Config;
use crate::phase0::configs::mainnet::{
    CHURN_LIMIT_QUOTIENT, EJECTION_BALANCE, ETH1_FOLLOW_DISTANCE, GENESIS_DELAY,
    GENESIS_FORK_VERSION, MIN_GENESIS_ACTIVE_VALIDATOR_COUNT, MIN_GENESIS_TIME,
    MIN_PER_EPOCH_CHURN_LIMIT, MIN_VALIDATOR_WITHDRAWABILITY_DELAY, SECONDS_PER_ETH1_BLOCK,
    SECONDS_PER_SLOT, SHARD_COMMITTEE_PERIOD,
};

pub const INACTIVITY_SCORE_BIAS: u64 = 4;
pub const INACTIVITY_SCORE_RECOVERY_RATE: u64 = 16;

pub const CONFIG: Config = Config {
    min_genesis_active_validator_count: MIN_GENESIS_ACTIVE_VALIDATOR_COUNT,
    min_genesis_time: MIN_GENESIS_TIME,
    genesis_fork_version: GENESIS_FORK_VERSION,
    genesis_delay: GENESIS_DELAY,
    seconds_per_slot: SECONDS_PER_SLOT,
    seconds_per_eth1_block: SECONDS_PER_ETH1_BLOCK,
    min_validator_withdrawability_delay: MIN_VALIDATOR_WITHDRAWABILITY_DELAY,
    shard_committee_period: SHARD_COMMITTEE_PERIOD,
    eth1_follow_distance: ETH1_FOLLOW_DISTANCE,
    ejection_balance: EJECTION_BALANCE,
    churn_limit_quotient: CHURN_LIMIT_QUOTIENT,
    min_per_epoch_churn_limit: MIN_PER_EPOCH_CHURN_LIMIT,
    inactivity_score_bias: INACTIVITY_SCORE_BIAS,
    inactivity_score_recovery_rate: INACTIVITY_SCORE_RECOVERY_RATE,
};
