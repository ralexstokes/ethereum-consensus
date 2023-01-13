use crate::primitives::{ExecutionAddress, Gwei, ValidatorIndex, WithdrawalIndex};
use ssz_rs::prelude::*;

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Withdrawal {
    pub index: WithdrawalIndex,
    pub validator_index: ValidatorIndex,
    pub address: ExecutionAddress,
    pub amount: Gwei,
}
