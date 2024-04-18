use crate::{
    phase0::Validator,
    primitives::{CommitteeIndex, Gwei, ValidatorIndex},
    ssz::prelude::*,
};

pub fn get_committee_indices<const MAX_COMMITTEES_PER_SLOT: usize>(
    committee_bits: Bitvector<MAX_COMMITTEES_PER_SLOT>,
) -> Vec<CommitteeIndex> {
    committee_bits.iter().enumerate().flat_map(|(i, bit)| bit.then_some(i)).collect()
}

pub fn get_activation_exit_churn_limit() -> Gwei {
    todo!()
}

pub fn get_consolidation_churn_limit() -> Gwei {
    todo!()
}

pub fn queue_entire_balance_and_reset_validator(_index: ValidatorIndex) {
    todo!()
}

pub fn has_compounding_withdrawal_credential(_validator: &Validator) -> bool {
    todo!()
}

pub fn queue_excess_active_balance(_index: ValidatorIndex) {
    todo!()
}
