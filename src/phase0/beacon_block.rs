use crate::primitives::{Root, Slot, ValidatorIndex};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconBlockHeader {
    slot: Slot,
    proposer_index: ValidatorIndex,
    parent_root: Root,
    state_root: Root,
    body_root: Root,
}
