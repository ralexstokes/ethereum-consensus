use crate::altair::presets::Preset;
use crate::altair::sync_committee;
pub use crate::phase0::mainnet::*;
use crate::primitives::Epoch;

pub const INACTIVITY_PENALTY_QUOTIENT_ALTAIR: u64 = 50331648;
pub const MIN_SLASHING_PENALTY_QUOTIENT_ALTAIR: u64 = 64;
pub const PROPORTIONAL_SLASHING_MULTIPLIER_ALTAIR: u64 = 2;
pub const SYNC_COMMITTEE_SIZE: usize = 512;
pub const EPOCHS_PER_SYNC_COMMITTEE_PERIOD: Epoch = 256;
pub const MIN_SYNC_COMMITTEE_PARTICIPANTS: usize = 1;
pub const UPDATE_TIMEOUT: usize = 8192;

pub const PRESET: Preset = Preset {
    inactivity_penalty_quotient_altair: INACTIVITY_PENALTY_QUOTIENT_ALTAIR,
    min_slashing_penalty_quotient_altair: MIN_SLASHING_PENALTY_QUOTIENT_ALTAIR,
    proportional_slashing_multiplier_altair: PROPORTIONAL_SLASHING_MULTIPLIER_ALTAIR,
    sync_committee_size: SYNC_COMMITTEE_SIZE,
    epochs_per_sync_committee_period: EPOCHS_PER_SYNC_COMMITTEE_PERIOD,
    min_sync_committee_participants: MIN_SYNC_COMMITTEE_PARTICIPANTS,
    update_timeout: UPDATE_TIMEOUT,
};

pub type SyncCommittee = sync_committee::SyncCommittee<SYNC_COMMITTEE_SIZE>;
