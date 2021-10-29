use crate::altair::{BeaconBlockHeader, SyncCommittee};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct Snapshot {
    pub header: BeaconBlockHeader,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: SyncCommittee,
}
