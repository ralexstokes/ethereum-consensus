mod presets;

use crate::primitives::{Bytes32, ExecutionAddress, Hash32, Root};
use ssz_rs::prelude::*;

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub type Transaction<const MAX_BYTES_PER_TRANSACTION: usize> = List<u8, MAX_BYTES_PER_TRANSACTION>;

#[derive(Default, Debug, SimpleSerialize)]
pub struct ExecutionPayload<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
> {
    parent_hash: Hash32,
    fee_recipient: ExecutionAddress,
    state_root: Bytes32,
    receipts_root: Bytes32,
    logs_bloom: Vector<u8, BYTES_PER_LOGS_BLOOM>,
    prev_randao: Bytes32,
    block_number: u64,
    gas_limit: u64,
    gas_used: u64,
    timestamp: u64,
    extra_data: List<u8, MAX_EXTRA_DATA_BYTES>,
    base_fee_per_gas: U256,
    block_hash: Hash32,
    transactions: List<Transaction<MAX_BYTES_PER_TRANSACTION>, MAX_TRANSACTIONS_PER_PAYLOAD>,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct ExecutionPayloadHeader<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
> {
    parent_hash: Hash32,
    fee_recipient: ExecutionAddress,
    state_root: Bytes32,
    receipts_root: Bytes32,
    logs_bloom: Vector<u8, BYTES_PER_LOGS_BLOOM>,
    prev_randao: Bytes32,
    block_number: u64,
    gas_limit: u64,
    gas_used: u64,
    timestamp: u64,
    extra_data: List<u8, MAX_EXTRA_DATA_BYTES>,
    base_fee_per_gas: U256,
    block_hash: Hash32,
    transactions_root: Root,
}
