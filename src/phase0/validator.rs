use crate::crypto::BLSPubkey;
use crate::primitives::{Bytes32, Epoch, Gwei};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct Validator {
    pubkey: BLSPubkey,
    withdrawal_credentials: Bytes32,
    effective_balance: Gwei,
    slashed: bool,
    // Status epochs
    activation_eligibility_epoch: Epoch,
    activation_epoch: Epoch,
    exit_epoch: Epoch,
    withdrawable_epoch: Epoch,
}
