use crate::crypto::BLSPubkey;
use crate::primitives::{Bytes32, Epoch, Gwei};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct Validator {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: Bytes32,
    pub effective_balance: Gwei,
    pub slashed: bool,
    // Status epochs
    pub activation_eligibility_epoch: Epoch,
    pub activation_epoch: Epoch,
    pub exit_epoch: Epoch,
    pub withdrawable_epoch: Epoch,
}
