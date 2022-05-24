use crate::primitives::{Bytes32, ExecutionAddress, Hash32, Root, U256};
use crate::ssz::{ByteList, ByteVector};
use ssz_rs::prelude::*;

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
