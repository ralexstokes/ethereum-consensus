//! This module provides an implementation of the `phase0` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
pub mod mainnet;
pub mod minimal;
mod operations;

use crate::crypto::{BLSPubkey, BLSSignature};
use crate::primitives::{
    Bytes32, CommitteeIndex, Domain, Epoch, Gwei, Hash32, Root, Slot, Version,
};
use ssz_rs::prelude::*;

// Required for bounds on the `BeaconState`.
pub(crate) const fn get_eth1_data_votes_bound(
    epochs_per_eth1_voting_period: usize,
    slots_per_epoch: usize,
) -> usize {
    epochs_per_eth1_voting_period * slots_per_epoch
}

// Required for bounds on the `BeaconState`.
pub(crate) const fn get_pending_attestations_bound(
    max_attestations: usize,
    slots_per_epoch: usize,
) -> usize {
    max_attestations * slots_per_epoch
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct Fork {
    pub previous_version: Version,
    pub current_version: Version,
    pub epoch: Epoch,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct ForkData {
    pub current_version: Version,
    pub genesis_validators_root: Root,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: Root,
}

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

#[derive(Default, Debug, SimpleSerialize)]
pub struct AttestationData {
    pub slot: Slot,
    pub index: CommitteeIndex,
    pub beacon_block_root: Root,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct Eth1Data {
    pub deposit_root: Root,
    pub deposit_count: u64,
    pub block_hash: Hash32,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct DepositMessage {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: Bytes32,
    pub amount: Gwei,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct DepositData {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: Bytes32,
    pub amount: Gwei,
    pub signature: BLSSignature,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SigningData {
    pub object_root: Root,
    pub domain: Domain,
}
