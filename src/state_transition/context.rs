use crate::configs::{mainnet, Config};
use crate::phase0::{mainnet::PRESET as MAINNET_PRESET, Preset};
use crate::primitives::{Epoch, Gwei, Slot, Version};

#[derive(Debug, Default)]
pub struct Context {
    pub max_committees_per_slot: u64,
    pub target_committee_size: u64,
    pub max_validators_per_committee: usize,
    pub shuffle_round_count: u64,
    pub hysteresis_quotient: u64,
    pub hysteresis_downward_multiplier: u64,
    pub hysteresis_upward_multiplier: u64,
    pub min_deposit_amount: Gwei,
    pub max_effective_balance: Gwei,
    pub effective_balance_increment: Gwei,
    pub min_attestation_inclusion_delay: Slot,
    pub slots_per_epoch: Slot,
    pub min_seed_lookahead: Epoch,
    pub max_seed_lookahead: Epoch,
    pub min_epochs_to_inactivity_penalty: Epoch,
    pub epochs_per_eth1_voting_period: Epoch,
    pub slots_per_historical_root: Slot,
    pub epochs_per_historical_vector: Epoch,
    pub epochs_per_slashings_vector: Epoch,
    pub historical_roots_limit: usize,
    pub validator_registry_limit: usize,
    pub base_reward_factor: u64,
    pub whistleblower_reward_quotient: u64,
    pub proposer_reward_quotient: u64,
    pub inactivity_penalty_quotient: u64,
    pub min_slashing_penalty_quotient: u64,
    pub proportional_slashing_multiplier: u64,
    pub max_proposer_slashings: usize,
    pub max_attester_slashings: usize,
    pub max_attestations: usize,
    pub max_deposits: usize,
    pub max_voluntary_exits: usize,
    pub min_genesis_active_validator_count: usize,
    pub min_genesis_time: u64,
    pub genesis_fork_version: Version,
    pub genesis_delay: u64,
    pub seconds_per_slot: u64,
    pub seconds_per_eth1_block: u64,
    pub min_validator_withdrawability_delay: Epoch,
    pub shard_committee_period: Epoch,
    pub eth1_follow_distance: Epoch,
    pub ejection_balance: Gwei,
    pub min_per_epoch_churn_limit: u64,
    pub churn_limit_quotient: u64,
}

impl Context {
    pub fn from(preset: &Preset, config: &Config) -> Self {
        Context {
            max_committees_per_slot: preset.max_committees_per_slot,
            target_committee_size: preset.target_committee_size,
            max_validators_per_committee: preset.max_validators_per_committee,
            shuffle_round_count: preset.shuffle_round_count,
            hysteresis_quotient: preset.hysteresis_quotient,
            hysteresis_downward_multiplier: preset.hysteresis_downward_multiplier,
            hysteresis_upward_multiplier: preset.hysteresis_upward_multiplier,
            min_deposit_amount: preset.min_deposit_amount,
            max_effective_balance: preset.max_effective_balance,
            effective_balance_increment: preset.effective_balance_increment,
            min_attestation_inclusion_delay: preset.min_attestation_inclusion_delay,
            slots_per_epoch: preset.slots_per_epoch,
            min_seed_lookahead: preset.min_seed_lookahead,
            max_seed_lookahead: preset.max_seed_lookahead,
            min_epochs_to_inactivity_penalty: preset.min_epochs_to_inactivity_penalty,
            epochs_per_eth1_voting_period: preset.epochs_per_eth1_voting_period,
            slots_per_historical_root: preset.slots_per_historical_root,
            epochs_per_historical_vector: preset.epochs_per_historical_vector,
            epochs_per_slashings_vector: preset.epochs_per_slashings_vector,
            historical_roots_limit: preset.historical_roots_limit,
            validator_registry_limit: preset.validator_registry_limit,
            base_reward_factor: preset.base_reward_factor,
            whistleblower_reward_quotient: preset.whistleblower_reward_quotient,
            proposer_reward_quotient: preset.proposer_reward_quotient,
            inactivity_penalty_quotient: preset.inactivity_penalty_quotient,
            min_slashing_penalty_quotient: preset.min_slashing_penalty_quotient,
            proportional_slashing_multiplier: preset.proportional_slashing_multiplier,
            max_proposer_slashings: preset.max_proposer_slashings,
            max_attester_slashings: preset.max_attester_slashings,
            max_attestations: preset.max_attestations,
            max_deposits: preset.max_deposits,
            max_voluntary_exits: preset.max_voluntary_exits,
            min_genesis_active_validator_count: config.min_genesis_active_validator_count,
            min_genesis_time: config.min_genesis_time,
            genesis_fork_version: config.genesis_fork_version,
            genesis_delay: config.genesis_delay,
            seconds_per_slot: config.seconds_per_slot,
            seconds_per_eth1_block: config.seconds_per_eth1_block,
            min_validator_withdrawability_delay: config.min_validator_withdrawability_delay,
            shard_committee_period: config.shard_committee_period,
            eth1_follow_distance: config.eth1_follow_distance,
            ejection_balance: config.ejection_balance,
            churn_limit_quotient: config.churn_limit_quotient,
            min_per_epoch_churn_limit: config.min_per_epoch_churn_limit,
        }
    }

    pub fn for_mainnet() -> Self {
        Self::from(&MAINNET_PRESET, &mainnet::config())
    }

    pub fn for_minimal() -> Self {
        unimplemented!()
    }
}
