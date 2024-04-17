use crate::{
    primitives::{BlsPublicKey, ExecutionAddress},
    ssz::prelude::*,
};

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct ExecutionLayerExit {
    pub source_address: ExecutionAddress,
    pub validator_pubkey: BlsPublicKey,
}
