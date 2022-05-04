use crate::primitives::{BlsSignature, Root, Slot, ValidatorIndex};
use ssz_rs::prelude::Bitvector;

pub const TARGET_AGGREGATORS_PER_SYNC_SUBCOMMITTEE: usize = 16;
pub const SYNC_COMMITTEE_SUBNET_COUNT: usize = 4;

pub struct SyncCommitteeMessage {
    pub slot: Slot,
    pub beacon_block_root: Root,
    pub validator_index: ValidatorIndex,
    pub signature: BlsSignature,
}

pub(super) const fn get_sync_subcommittee_size(sync_committee_size: usize) -> usize {
    sync_committee_size / SYNC_COMMITTEE_SUBNET_COUNT
}

pub struct SyncCommitteeContribution<const SYNC_SUBCOMMITTEE_SIZE: usize> {
    pub slot: Slot,
    pub beacon_block_root: Root,
    pub subcommittee_index: u64,
    pub aggregation_bits: Bitvector<SYNC_SUBCOMMITTEE_SIZE>,
    pub signature: BlsSignature,
}

pub struct ContributionAndProof<const SYNC_SUBCOMMITTEE_SIZE: usize> {
    pub aggregator_index: ValidatorIndex,
    pub contribution: SyncCommitteeContribution<SYNC_SUBCOMMITTEE_SIZE>,
    pub selection_proof: BlsSignature,
}

pub struct SignedContributionAndProof<const SYNC_SUBCOMMITTEE_SIZE: usize> {
    pub message: ContributionAndProof<SYNC_SUBCOMMITTEE_SIZE>,
    pub signature: BlsSignature,
}
