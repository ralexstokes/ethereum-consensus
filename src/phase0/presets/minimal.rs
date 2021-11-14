use crate::primitives::{Epoch, Gwei, Slot};

pub const MAX_COMMITTEES_PER_SLOT: u64 = 4;
pub const TARGET_COMMITTEE_SIZE: u64 = 4;
pub const MAX_VALIDATORS_PER_COMMITTEE: usize = 2048;
pub const SHUFFLE_ROUND_COUNT: u64 = 10;
pub const HYSTERESIS_QUOTIENT: u64 = 4;
pub const HYSTERESIS_DOWNWARD_MULTIPLIER: u64 = 1;
pub const HYSTERESIS_UPWARD_MULTIPLIER: u64 = 5;
pub const MIN_DEPOSIT_AMOUNT: Gwei = 10u64.pow(9);
pub const MAX_EFFECTIVE_BALANCE: Gwei = 32 * 10u64.pow(9);
pub const EFFECTIVE_BALANCE_INCREMENT: Gwei = 10u64.pow(9);
pub const MIN_ATTESTATION_INCLUSION_DELAY: Slot = 1;
pub const SLOTS_PER_EPOCH: u64 = 8;
pub const MIN_SEED_LOOKAHEAD: Epoch = 1;
pub const MAX_SEED_LOOKAHEAD: Epoch = 4;
pub const MIN_EPOCHS_TO_INACTIVITY_PENALTY: Epoch = 4;
pub const EPOCHS_PER_ETH1_VOTING_PERIOD: usize = 4;
pub const SLOTS_PER_HISTORICAL_ROOT: usize = 64;
pub const EPOCHS_PER_HISTORICAL_VECTOR: usize = 64;
pub const EPOCHS_PER_SLASHINGS_VECTOR: usize = 64;
pub const HISTORICAL_ROOTS_LIMIT: usize = 16_777_216;
pub const VALIDATOR_REGISTRY_LIMIT: usize = 2usize.pow(40);
pub const BASE_REWARD_FACTOR: u64 = 64;
pub const WHISTLEBLOWER_REWARD_QUOTIENT: u64 = 512;
pub const PROPOSER_REWARD_QUOTIENT: u64 = 8;
pub const INACTIVITY_PENALTY_QUOTIENT: u64 = 2u64.pow(25);
pub const MIN_SLASHING_PENALTY_QUOTIENT: u64 = 64;
pub const PROPORTIONAL_SLASHING_MULTIPLIER: u64 = 2;
pub const MAX_PROPOSER_SLASHINGS: usize = 16;
pub const MAX_ATTESTER_SLASHINGS: usize = 2;
pub const MAX_ATTESTATIONS: usize = 128;
pub const MAX_DEPOSITS: usize = 16;
pub const MAX_VOLUNTARY_EXITS: usize = 16;

// TODO: specialize once `mainnet` has stabilized
