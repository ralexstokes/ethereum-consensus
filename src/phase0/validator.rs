use crate::phase0::operations::Attestation;
use crate::primitives::{BlsPublicKey, BlsSignature, Bytes32, Epoch, Gwei, ValidatorIndex};
use ssz_rs::prelude::*;

pub const TARGET_AGGREGATORS_PER_COMMITTEE: usize = 16;
pub const RANDOM_SUBNETS_PER_VALIDATOR: usize = 1;
pub const EPOCHS_PER_RANDOM_SUBNET_SUBSCRIPTION: Epoch = 256;

#[derive(Default, Debug, SimpleSerialize, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Validator {
    pub pubkey: BlsPublicKey,
    pub withdrawal_credentials: Bytes32,
    #[serde(with = "crate::serde::as_string")]
    pub effective_balance: Gwei,
    pub slashed: bool,
    // Status epochs
    #[serde(with = "crate::serde::as_string")]
    pub activation_eligibility_epoch: Epoch,
    #[serde(with = "crate::serde::as_string")]
    pub activation_epoch: Epoch,
    #[serde(with = "crate::serde::as_string")]
    pub exit_epoch: Epoch,
    #[serde(with = "crate::serde::as_string")]
    pub withdrawable_epoch: Epoch,
}

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AggregateAndProof<const MAX_VALIDATORS_PER_COMMITTEE: usize> {
    #[serde(with = "crate::serde::as_string")]
    pub aggregator_index: ValidatorIndex,
    pub aggregate: Attestation<MAX_VALIDATORS_PER_COMMITTEE>,
    pub selection_proof: BlsSignature,
}

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedAggregateAndProof<const MAX_VALIDATORS_PER_COMMITTEE: usize> {
    pub message: AggregateAndProof<MAX_VALIDATORS_PER_COMMITTEE>,
    pub signature: BlsSignature,
}
