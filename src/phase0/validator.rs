use crate::phase0::operations::Attestation;
use crate::primitives::{BlsPublicKey, BlsSignature, Bytes32, Epoch, Gwei, ValidatorIndex};
use ssz_rs::prelude::*;

pub const TARGET_AGGREGATORS_PER_COMMITTEE: usize = 16;
pub const RANDOM_SUBNETS_PER_VALIDATOR: usize = 1;
pub const EPOCHS_PER_RANDOM_SUBNET_SUBSCRIPTION: Epoch = 256;

#[derive(Default, Debug, SimpleSerialize, Clone)]
pub struct Validator {
    pub pubkey: BlsPublicKey,
    pub withdrawal_credentials: Bytes32,
    pub effective_balance: Gwei,
    pub slashed: bool,
    // Status epochs
    pub activation_eligibility_epoch: Epoch,
    pub activation_epoch: Epoch,
    pub exit_epoch: Epoch,
    pub withdrawable_epoch: Epoch,
}

pub struct AggregateAndProof<const MAX_VALIDATORS_PER_COMMITTEE: usize> {
    aggregator_index: ValidatorIndex,
    aggregate: Attestation<MAX_VALIDATORS_PER_COMMITTEE>,
    selection_proof: BlsSignature,
}

pub struct SignedAggregateAndProof<const MAX_VALIDATORS_PER_COMMITTEE: usize> {
    message: AggregateAndProof<MAX_VALIDATORS_PER_COMMITTEE>,
    signature: BlsSignature,
}
