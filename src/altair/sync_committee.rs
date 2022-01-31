use crate::altair::presets::mainnet::SYNC_COMMITTEE_SIZE;
use crate::crypto::PublicKey as BLSPubkey;
use ssz_rs::{prelude::*, Vector};

#[derive(Default, Debug, SimpleSerialize, Clone)]
pub struct SyncCommittee {
    pub pubkeys: Vector<BLSPubkey, SYNC_COMMITTEE_SIZE>,
    pub aggregate_pubkey: BLSPubkey,
}
