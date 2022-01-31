use crate::altair::beacon_block::BeaconBlockHeader;
use crate::altair::sync_committee::SyncCommittee;
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct Snapshot {
    pub header: BeaconBlockHeader,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: SyncCommittee,
}
