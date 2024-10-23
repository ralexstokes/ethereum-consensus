use crate::{
    altair::mainnet::SYNC_COMMITTEE_SIZE,
    bellatrix::mainnet::{
        BYTES_PER_LOGS_BLOOM, MAX_BYTES_PER_TRANSACTION, MAX_EXTRA_DATA_BYTES,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    },
    capella::mainnet::{MAX_BLS_TO_EXECUTION_CHANGES, MAX_WITHDRAWALS_PER_PAYLOAD},
    deneb::mainnet::MAX_BLOB_COMMITMENTS_PER_BLOCK,
    electra::mainnet::{
        MAX_ATTESTATIONS_ELECTRA, MAX_ATTESTER_SLASHINGS_ELECTRA, MAX_COMMITTEES_PER_SLOT,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD, MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_VALIDATORS_PER_SLOT, MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    },
    phase0::mainnet::{
        EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR, ETH1_DATA_VOTES_BOUND,
        HISTORICAL_ROOTS_LIMIT, MAX_ATTESTATIONS, MAX_ATTESTER_SLASHINGS, MAX_DEPOSITS,
        MAX_PROPOSER_SLASHINGS, MAX_VALIDATORS_PER_COMMITTEE, MAX_VOLUNTARY_EXITS,
        PENDING_ATTESTATIONS_BOUND, SLOTS_PER_HISTORICAL_ROOT, VALIDATOR_REGISTRY_LIMIT,
    },
    state_transition,
};

pub use crate::Error;
pub use state_transition::{Context, Validation};

pub type Executor = state_transition::Executor<
    SLOTS_PER_HISTORICAL_ROOT,
    HISTORICAL_ROOTS_LIMIT,
    ETH1_DATA_VOTES_BOUND,
    VALIDATOR_REGISTRY_LIMIT,
    EPOCHS_PER_HISTORICAL_VECTOR,
    EPOCHS_PER_SLASHINGS_VECTOR,
    MAX_VALIDATORS_PER_COMMITTEE,
    PENDING_ATTESTATIONS_BOUND,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_PROPOSER_SLASHINGS,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOB_COMMITMENTS_PER_BLOCK,
    MAX_VALIDATORS_PER_SLOT,
    MAX_COMMITTEES_PER_SLOT,
    MAX_ATTESTER_SLASHINGS_ELECTRA,
    MAX_ATTESTATIONS_ELECTRA,
    MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
    MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
    MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
>;
