pub use crate::domains::{DomainType, SigningData};
use crate::phase0::beacon_block;
pub use crate::phase0::beacon_block::{BeaconBlockHeader, SignedBeaconBlockHeader};
use crate::phase0::beacon_state;
use crate::phase0::beacon_state::{get_eth1_data_votes_bound, get_pending_attestations_bound};
pub use crate::phase0::fork::{Fork, ForkData};
use crate::phase0::operations;
pub use crate::phase0::operations::{
    AttestationData, Checkpoint, Deposit, DepositData, DepositMessage, Eth1Data, ProposerSlashing,
    SignedVoluntaryExit, VoluntaryExit,
};
pub use crate::phase0::validator::Validator;
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

pub type IndexedAttestation = operations::IndexedAttestation<MAX_VALIDATORS_PER_COMMITTEE>;
pub type PendingAttestation = operations::PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>;
pub type HistoricalBatch = beacon_state::HistoricalBatch<SLOTS_PER_HISTORICAL_ROOT>;
pub type AttesterSlashing = operations::AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>;
pub type Attestation = operations::Attestation<MAX_VALIDATORS_PER_COMMITTEE>;

const ETH1_DATA_VOTES_BOUND: usize =
    get_eth1_data_votes_bound(EPOCHS_PER_ETH1_VOTING_PERIOD, SLOTS_PER_EPOCH as usize);
const PENDING_ATTESTATIONS_BOUND: usize =
    get_pending_attestations_bound(MAX_ATTESTATIONS, SLOTS_PER_EPOCH as usize);

pub type BeaconState = beacon_state::BeaconState<
    SLOTS_PER_HISTORICAL_ROOT,
    HISTORICAL_ROOTS_LIMIT,
    ETH1_DATA_VOTES_BOUND,
    VALIDATOR_REGISTRY_LIMIT,
    EPOCHS_PER_HISTORICAL_VECTOR,
    EPOCHS_PER_SLASHINGS_VECTOR,
    MAX_VALIDATORS_PER_COMMITTEE,
    PENDING_ATTESTATIONS_BOUND,
>;

pub type BeaconBlockBody = beacon_block::BeaconBlockBody<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
>;

pub type BeaconBlock = beacon_block::BeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
>;

pub type SignedBeaconBlock = beacon_block::SignedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
>;
