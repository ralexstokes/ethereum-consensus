use crate::{
    crypto::BLSSignature,
    primitives::{Root, Slot, ValidatorIndex},
};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconBlockHeader {
    pub slot: Slot,
    pub proposer_index: ValidatorIndex,
    pub parent_root: Root,
    pub state_root: Root,
    pub body_root: Root,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SignedBeaconBlockHeader {
    pub message: BeaconBlockHeader,
    pub signature: BLSSignature,
}
