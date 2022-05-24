use crate::altair::sync::SyncCommittee;
use crate::phase0::BeaconBlockHeader;
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Snapshot<const SYNC_COMMITTEE_SIZE: usize> {
    pub header: BeaconBlockHeader,
    pub current_sync_committee: SyncCommittee<SYNC_COMMITTEE_SIZE>,
    pub next_sync_committee: SyncCommittee<SYNC_COMMITTEE_SIZE>,
}
