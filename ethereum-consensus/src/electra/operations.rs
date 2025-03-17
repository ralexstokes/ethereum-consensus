use crate::{
    phase0::AttestationData,
    primitives::{BlsSignature, ValidatorIndex},
    ssz::prelude::*,
};

use super::{BlsPublicKey, Bytes32, ExecutionAddress, Gwei};

#[derive(
    Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct AttesterSlashing<const MAX_VALIDATORS_PER_SLOT: usize> {
    pub attestation_1: IndexedAttestation<MAX_VALIDATORS_PER_SLOT>,
    pub attestation_2: IndexedAttestation<MAX_VALIDATORS_PER_SLOT>,
}

#[derive(
    Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct IndexedAttestation<const MAX_VALIDATORS_PER_SLOT: usize> {
    #[serde(with = "crate::serde::seq_of_str")]
    pub attesting_indices: List<ValidatorIndex, MAX_VALIDATORS_PER_SLOT>,
    pub data: AttestationData,
    pub signature: BlsSignature,
}

#[derive(
    Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct Attestation<const MAX_VALIDATORS_PER_SLOT: usize, const MAX_COMMITTEES_PER_SLOT: usize> {
    pub aggregation_bits: Bitlist<MAX_VALIDATORS_PER_SLOT>,
    pub data: AttestationData,
    pub signature: BlsSignature,
    pub committee_bits: Bitvector<MAX_COMMITTEES_PER_SLOT>,
}

#[derive(
    Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct DepositRequest {
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
    pub withdrawal_credentials: Bytes32,
    #[serde(with = "crate::serde::as_str")]
    pub amount: Gwei,
    pub signature: BlsSignature,
    #[serde(with = "crate::serde::as_str")]
    pub index: u64,
}

#[derive(
    Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct WithdrawalRequest {
    pub source_address: ExecutionAddress,
    pub validator_pubkey: BlsPublicKey,
    #[serde(with = "crate::serde::as_str")]
    pub amount: Gwei,
}

#[derive(
    Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct ConsolidationRequest {
    pub source_address: ExecutionAddress,
    pub source_pubkey: BlsPublicKey,
    pub target_pubkey: BlsPublicKey,
}

#[derive(
    Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct ExecutionRequests<
    const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
    const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
> {
    pub deposits: List<DepositRequest, MAX_DEPOSIT_REQUESTS_PER_PAYLOAD>,
    pub withdrawals: List<WithdrawalRequest, MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD>,
    pub consolidations: List<ConsolidationRequest, MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD>,
}
