use crate::{
    primitives::{ExecutionAddress, Gwei, ValidatorIndex, WithdrawalIndex},
    ssz::prelude::*,
};

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct Withdrawal {
    #[serde(with = "crate::serde::as_str")]
    pub index: WithdrawalIndex,
    #[serde(with = "crate::serde::as_str")]
    pub validator_index: ValidatorIndex,
    pub address: ExecutionAddress,
    #[serde(with = "crate::serde::as_str")]
    pub amount: Gwei,
}
