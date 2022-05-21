mod presets;

use crate::altair::sync::SyncAggregate;
use crate::phase0::operations::{
    Attestation, AttesterSlashing, Deposit, Eth1Data, ProposerSlashing, SignedVoluntaryExit,
};
use crate::ssz::{ByteList, ByteVector};

use crate::primitives::{
    BlsSignature, Bytes32, ExecutionAddress, Hash32, Root, Slot, ValidatorIndex,
};
use ssz_rs::prelude::*;

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub type Transaction<const MAX_BYTES_PER_TRANSACTION: usize> = ByteList<MAX_BYTES_PER_TRANSACTION>;

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecutionPayload<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
> {
    pub parent_hash: Hash32,
    pub fee_recipient: ExecutionAddress,
    pub state_root: Bytes32,
    pub receipts_root: Bytes32,
    pub logs_bloom: ByteVector<BYTES_PER_LOGS_BLOOM>,
    pub prev_randao: Bytes32,
    #[serde(with = "crate::serde::as_string")]
    pub block_number: u64,
    #[serde(with = "crate::serde::as_string")]
    pub gas_limit: u64,
    #[serde(with = "crate::serde::as_string")]
    pub gas_used: u64,
    #[serde(with = "crate::serde::as_string")]
    pub timestamp: u64,
    pub extra_data: ByteList<MAX_EXTRA_DATA_BYTES>,
    pub base_fee_per_gas: U256,
    pub block_hash: Hash32,
    pub transactions: List<Transaction<MAX_BYTES_PER_TRANSACTION>, MAX_TRANSACTIONS_PER_PAYLOAD>,
}

#[derive(Default, Debug, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecutionPayloadHeader<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
> {
    pub parent_hash: Hash32,
    pub fee_recipient: ExecutionAddress,
    pub state_root: Bytes32,
    pub receipts_root: Bytes32,
    pub logs_bloom: ByteVector<BYTES_PER_LOGS_BLOOM>,
    pub prev_randao: Bytes32,
    #[serde(with = "crate::serde::as_string")]
    pub block_number: u64,
    #[serde(with = "crate::serde::as_string")]
    pub gas_limit: u64,
    #[serde(with = "crate::serde::as_string")]
    pub gas_used: u64,
    #[serde(with = "crate::serde::as_string")]
    pub timestamp: u64,
    pub extra_data: ByteList<MAX_EXTRA_DATA_BYTES>,
    pub base_fee_per_gas: U256,
    pub block_hash: Hash32,
    pub transactions_root: Root,
}

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlindedBeaconBlockBody<
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
> {
    pub randao_reveal: BlsSignature,
    pub eth1_data: Eth1Data,
    pub graffiti: Bytes32,
    pub proposer_slashings: List<ProposerSlashing, MAX_PROPOSER_SLASHINGS>,
    pub attester_slashings:
        List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS>,
    pub attestations: List<Attestation<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTATIONS>,
    pub deposits: List<Deposit, MAX_DEPOSITS>,
    pub voluntary_exits: List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS>,
    pub sync_aggregate: SyncAggregate<SYNC_COMMITTEE_SIZE>,
    pub execution_payload_header: ExecutionPayloadHeader<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
}

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlindedBeaconBlock<
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
> {
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    #[serde(with = "crate::serde::as_string")]
    pub proposer_index: ValidatorIndex,
    pub parent_root: Root,
    pub state_root: Root,
    pub body: BlindedBeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
}

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedBlindedBeaconBlock<
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
> {
    pub message: BlindedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    pub signature: BlsSignature,
}
