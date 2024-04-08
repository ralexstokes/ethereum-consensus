use crate::{
    primitives::{BlsPublicKey, BlsSignature, Bytes32, Gwei},
    ssz::prelude::*,
};

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct DepositReceipt {
    pub pubkey: BlsPublicKey,
    pub withdrawal_credentials: Bytes32,
    #[serde(with = "crate::serde::as_str")]
    pub amount: Gwei,
    pub signature: BlsSignature,
    #[serde(with = "crate::serde::as_str")]
    pub index: u64,
}
