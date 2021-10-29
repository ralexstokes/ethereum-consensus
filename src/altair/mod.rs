pub mod light_client;

use crate::{crypto::BLSPubkey, phase0};
use ssz_rs::prelude::*;

pub type BeaconBlockHeader = phase0::BeaconBlockHeader;

pub const SYNC_COMMITTEE_SIZE: usize = 512;

#[derive(Default, Debug, SimpleSerialize)]
pub struct SyncCommittee {
    pub pubkeys: Vector<BLSPubkey, SYNC_COMMITTEE_SIZE>,
    pub aggregate_pubkey: BLSPubkey,
}
