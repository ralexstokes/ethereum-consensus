use crate::lib::*;
use crate::primitives::{BlsPublicKey, BlsSignature, ExecutionAddress, ValidatorIndex};
use ssz_rs::prelude::*;

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlsToExecutionChange {
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::as_string"))]
    pub validator_index: ValidatorIndex,
    #[cfg_attr(feature = "serde", serde(rename = "from_bls_pubkey"))]
    pub from_bls_public_key: BlsPublicKey,
    pub to_execution_address: ExecutionAddress,
}

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedBlsToExecutionChange {
    message: BlsToExecutionChange,
    signature: BlsSignature,
}
