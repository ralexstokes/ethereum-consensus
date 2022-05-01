use crate::bellatrix::presets::Preset;
use crate::primitives::Epoch;

pub const INACTIVITY_PENALTY_QUOTIENT_BELLATRIX: u64 = 16777216;
pub const MIN_SLASHING_PENALTY_QUOTIENT_BELLATRIX: u64 = 32;
pub const PROPORTIONAL_SLASHING_MULTIPLIER_BELLATRIX: u64 = 3;
pub const MAX_BYTES_PER_TRANSACTION: usize = 1073741824;
pub const MAX_TRANSACTIONS_PER_PAYLOAD: usize = 1048576;
pub const BYTES_PER_LOG_BLOOM: usize = 256;
pub const MAX_EXTRA_DATA_BYTES: usize = 32;

pub const PRESET: Preset = Preset {
    inactivity_penalty_quotient_bellatrix: INACTIVITY_PENALTY_QUOTIENT_BELLATRIX,
    min_slashing_penalty_quotient_bellatrix: MIN_SLASHING_PENALTY_QUOTIENT_BELLATRIX,
    proportional_slashing_multiplier_bellatrix: PROPORTIONAL_SLASHING_MULTIPLIER_BELLATRIX,
    max_bytes_per_transaction: MAX_BYTES_PER_TRANSACTION,
    max_transactions_per_payload: MAX_TRANSACTIONS_PER_PAYLOAD,
    bytes_per_logs_bloom: BYTES_PER_LOGS_BLOOM,
    max_extra_data_bytes: MAX_EXTRA_DATA_BYTES,
};
