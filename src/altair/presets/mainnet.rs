use crate::altair::presets::Preset;
use crate::altair::sync;
use crate::altair::validator;
use crate::primitives::Epoch;

pub use validator::SyncCommitteeMessage;

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

pub const SYNC_SUBCOMMITTEE_SIZE: usize =
    validator::get_sync_subcommittee_size(SYNC_COMMITTEE_SIZE);

pub type SyncAggregate = sync::SyncAggregate<SYNC_COMMITTEE_SIZE>;
pub type SyncCommittee = sync::SyncCommittee<SYNC_COMMITTEE_SIZE>;

pub type SyncCommitteeContribution = validator::SyncCommitteeContribution<SYNC_SUBCOMMITTEE_SIZE>;
pub type ContributionAndProof = validator::ContributionAndProof<SYNC_SUBCOMMITTEE_SIZE>;
pub type SignedContributionAndProof = validator::SignedContributionAndProof<SYNC_SUBCOMMITTEE_SIZE>;
