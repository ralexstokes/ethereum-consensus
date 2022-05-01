use crate::crypto::PublicKey as BLSPubkey;
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct SyncCommittee<const SYNC_COMMITTEE_SIZE: usize> {
    pub pubkeys: Vector<BLSPubkey, SYNC_COMMITTEE_SIZE>,
    pub aggregate_pubkey: BLSPubkey,
}
