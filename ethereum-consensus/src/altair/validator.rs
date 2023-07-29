use crate::primitives::{BlsSignature, Root, Slot, ValidatorIndex};
use ssz_rs::prelude::*;

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncCommitteeMessage {
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    pub beacon_block_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    pub signature: BlsSignature,
}

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncCommitteeContribution<const SYNC_SUBCOMMITTEE_SIZE: usize> {
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    pub beacon_block_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub subcommittee_index: u64,
    pub aggregation_bits: Bitvector<SYNC_SUBCOMMITTEE_SIZE>,
    pub signature: BlsSignature,
}

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContributionAndProof<const SYNC_SUBCOMMITTEE_SIZE: usize> {
    #[serde(with = "crate::serde::as_string")]
    pub aggregator_index: ValidatorIndex,
    pub contribution: SyncCommitteeContribution<SYNC_SUBCOMMITTEE_SIZE>,
    pub selection_proof: BlsSignature,
}

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedContributionAndProof<const SYNC_SUBCOMMITTEE_SIZE: usize> {
    pub message: ContributionAndProof<SYNC_SUBCOMMITTEE_SIZE>,
    pub signature: BlsSignature,
}

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncAggregatorSelectionData {
    pub slot: Slot,
    pub subcommittee_index: u64,
}
