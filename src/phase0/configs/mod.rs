pub mod mainnet;
pub mod minimal;

use crate::primitives::{Epoch, Gwei};

pub struct Config {
    pub min_genesis_active_validator_counts: u64,
    pub min_genesis_time: u64,
    pub genesis_delay: u64,
    pub seconds_per_slot: u64,
    pub seconds_per_eth1_block: u64,
    pub min_validator_withdrawability_delay: Epoch,
    pub shard_committee_period: Epoch,
    pub eth1_follow_distance: u64,
    pub ejection_balance: Gwei,
    pub churn_limit_quotient: u64,
    pub min_per_epoch_churn_limit: u64,
}
