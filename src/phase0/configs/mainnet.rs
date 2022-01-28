use crate::phase0::configs::Config;
use crate::primitives::{Epoch, Gwei, Version};

pub const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: usize = 16384;
pub const MIN_GENESIS_TIME: u64 = 1606824000;
pub const GENESIS_FORK_VERSION: Version = [0u8; 4];
pub const GENESIS_DELAY: u64 = 604800;
pub const SECONDS_PER_SLOT: u64 = 12;
pub const SECONDS_PER_ETH1_BLOCK: u64 = 14;
pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: Epoch = 256;
pub const SHARD_COMMITTEE_PERIOD: Epoch = 256;
pub const ETH1_FOLLOW_DISTANCE: u64 = 2048;
pub const EJECTION_BALANCE: Gwei = 16 * 10u64.pow(9);
pub const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 4;
pub const CHURN_LIMIT_QUOTIENT: u64 = 65536;

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
};
