use crate::primitives::{Epoch, Root, Version};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct Fork {
    pub previous_version: Version,
    pub current_version: Version,
    pub epoch: Epoch,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct ForkData {
    pub current_version: Version,
    pub genesis_validators_root: Root,
}
