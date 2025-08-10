use crate::{
    phase0::AttestationData,
    primitives::{BlsPublicKey, BlsSignature, Bytes32, ExecutionAddress, Gwei, ValidatorIndex},
    ssz::prelude::*,
};

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
    pub committee_bits: Bitvector<MAX_COMMITTEES_PER_SLOT>,
    pub signature: BlsSignature,
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
    #[serde(rename = "validator_pubkey")]
    pub validator_public_key: BlsPublicKey,
    #[serde(with = "crate::serde::as_str")]
    pub amount: Gwei,
}

#[derive(
    Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct ConsolidationRequest {
    pub source_address: ExecutionAddress,
    #[serde(rename = "source_pubkey")]
    pub source_public_key: BlsPublicKey,
    #[serde(rename = "target_pubkey")]
    pub target_public_key: BlsPublicKey,
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

// #[derive(
//     Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
// )]
// pub struct Consolidation {
//     #[serde(with = "crate::serde::as_str")]
//     pub source_index: ValidatorIndex,
//     #[serde(with = "crate::serde::as_str")]
//     pub target_index: ValidatorIndex,
//     #[serde(with = "crate::serde::as_str")]
//     pub epoch: Epoch,
// }

// #[derive(
//     Default, Debug, SimpleSerialize, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
// )]
// pub struct SignedConsolidation {
//     pub message: Consolidation,
//     pub signature: BlsSignature,
// }
