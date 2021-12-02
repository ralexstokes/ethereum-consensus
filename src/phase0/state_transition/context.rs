use crate::phase0::presets::Preset;
use crate::primitives::{Epoch, Gwei, Slot};
use ssz_rs::prelude::MerkleizationContext;

#[derive(Debug, Default)]
pub struct Context {
    merkleization_context: MerkleizationContext,
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
    pub epochs_per_eth1_voting_period: usize,
    pub slots_per_historical_root: usize,
    pub epochs_per_historical_vector: usize,
    pub epochs_per_slashings_vector: usize,
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
    pub min_validator_withdrawability_delay: Epoch,
    pub churn_limit_quatient: u64,
    pub min_per_epoch_churn_limit: u64,
}

impl Context {
    pub fn with_preset(preset: &Preset) -> Self {
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
            churn_limit_quatient: preset.churn_limit_quatient,
            min_validator_withdrawability_delay: preset.min_validator_withdrawability_delay,
            min_per_epoch_churn_limit: preset.min_per_epoch_churn_limit,
            ..Default::default()
        }
    }

    pub fn for_merkleization(&self) -> &MerkleizationContext {
        &self.merkleization_context
    }
}
