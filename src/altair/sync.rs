use crate::primitives::{BlsPublicKey, BlsSignature};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncAggregate<const SYNC_COMMITTEE_SIZE: usize> {
    pub sync_committee_bits: Bitvector<SYNC_COMMITTEE_SIZE>,
    pub sync_committee_signature: BlsSignature,
}

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncCommittee<const SYNC_COMMITTEE_SIZE: usize> {
    pub pubkeys: Vector<BlsPublicKey, SYNC_COMMITTEE_SIZE>,
    pub aggregate_pubkey: BlsPublicKey,
}
