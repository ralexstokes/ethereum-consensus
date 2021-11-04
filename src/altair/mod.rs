pub mod light_client;

use crate::{crypto::PublicKey as BLSPubkey, phase0};
use ssz_rs::prelude::*;

pub type BeaconBlockHeader = phase0::mainnet::BeaconBlockHeader;

pub const SYNC_COMMITTEE_SIZE: usize = 512;

#[derive(Default, Debug, SimpleSerialize)]
pub struct SyncCommittee {
    pub pubkeys: Vector<BLSPubkey, SYNC_COMMITTEE_SIZE>,
    pub aggregate_pubkey: BLSPubkey,
}
