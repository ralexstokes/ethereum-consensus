use crate::{
    primitives::{ExecutionAddress, Gwei, ValidatorIndex, WithdrawalIndex},
    ssz::prelude::*,
};

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Withdrawal {
    #[serde(with = "crate::serde::as_string")]
    pub index: WithdrawalIndex,
    #[serde(with = "crate::serde::as_string")]
    pub validator_index: ValidatorIndex,
    pub address: ExecutionAddress,
    #[serde(with = "crate::serde::as_string")]
    pub amount: Gwei,
}
