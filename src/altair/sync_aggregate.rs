use crate::altair::presets::mainnet::SYNC_COMMITTEE_SIZE;
use crate::crypto::Signature as BLSSignature;
use ssz_rs::{prelude::*, Bitvector};

#[derive(Default, Debug, SimpleSerialize, Clone)]
pub struct SyncAggregate {
    pub sync_committee_bits: Bitvector<SYNC_COMMITTEE_SIZE>,
    pub sync_committee_signature: BLSSignature,
}
