use crate::{primitives::Hash32, ssz::prelude::*};

#[derive(Default, Debug, SimpleSerialize, Clone, serde::Serialize, serde::Deserialize)]
pub struct PowBlock {
    block_hash: Hash32,
    parent_hash: Hash32,
    #[serde(with = "crate::serde::as_str")]
    total_difficulty: U256,
}
