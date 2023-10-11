//! WARNING: This file was derived by the `spec-gen` utility. DO NOT EDIT MANUALLY.
use crate::{
    bellatrix::execution_payload::{self as bellatrix, Transaction},
    capella::{execution_payload as capella, withdrawal::Withdrawal},
    deneb::execution_payload as deneb,
    primitives::{Bytes32, ExecutionAddress, Hash32},
    ssz::prelude::*,
    Fork as Version,
};
#[derive(Debug, Clone, PartialEq, Eq, SimpleSerialize, serde::Deserialize)]
#[serde(tag = "version", content = "data")]
#[serde(rename_all = "lowercase")]
pub enum ExecutionPayload<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
> {
    Bellatrix(
        bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ),
    Capella(
        capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ),
    Deneb(
        deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ),
}
impl<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    ExecutionPayload<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn bellatrix_mut(
        &mut self,
    ) -> Option<
        &mut bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella(
        &self,
    ) -> Option<
        &capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella_mut(
        &mut self,
    ) -> Option<
        &mut capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb(
        &self,
    ) -> Option<
        &deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb_mut(
        &mut self,
    ) -> Option<
        &mut deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
        }
    }
    pub fn parent_hash(&self) -> Option<&Hash32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.parent_hash),
            Self::Capella(inner) => Some(&inner.parent_hash),
            Self::Deneb(inner) => Some(&inner.parent_hash),
        }
    }
    pub fn parent_hash_mut(&mut self) -> Option<&mut Hash32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.parent_hash),
            Self::Capella(inner) => Some(&mut inner.parent_hash),
            Self::Deneb(inner) => Some(&mut inner.parent_hash),
        }
    }
    pub fn fee_recipient(&self) -> Option<&ExecutionAddress> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.fee_recipient),
            Self::Capella(inner) => Some(&inner.fee_recipient),
            Self::Deneb(inner) => Some(&inner.fee_recipient),
        }
    }
    pub fn fee_recipient_mut(&mut self) -> Option<&mut ExecutionAddress> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.fee_recipient),
            Self::Capella(inner) => Some(&mut inner.fee_recipient),
            Self::Deneb(inner) => Some(&mut inner.fee_recipient),
        }
    }
    pub fn state_root(&self) -> Option<&Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.state_root),
            Self::Capella(inner) => Some(&inner.state_root),
            Self::Deneb(inner) => Some(&inner.state_root),
        }
    }
    pub fn state_root_mut(&mut self) -> Option<&mut Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.state_root),
            Self::Capella(inner) => Some(&mut inner.state_root),
            Self::Deneb(inner) => Some(&mut inner.state_root),
        }
    }
    pub fn receipts_root(&self) -> Option<&Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.receipts_root),
            Self::Capella(inner) => Some(&inner.receipts_root),
            Self::Deneb(inner) => Some(&inner.receipts_root),
        }
    }
    pub fn receipts_root_mut(&mut self) -> Option<&mut Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.receipts_root),
            Self::Capella(inner) => Some(&mut inner.receipts_root),
            Self::Deneb(inner) => Some(&mut inner.receipts_root),
        }
    }
    pub fn logs_bloom(&self) -> Option<&ByteVector<BYTES_PER_LOGS_BLOOM>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.logs_bloom),
            Self::Capella(inner) => Some(&inner.logs_bloom),
            Self::Deneb(inner) => Some(&inner.logs_bloom),
        }
    }
    pub fn logs_bloom_mut(&mut self) -> Option<&mut ByteVector<BYTES_PER_LOGS_BLOOM>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.logs_bloom),
            Self::Capella(inner) => Some(&mut inner.logs_bloom),
            Self::Deneb(inner) => Some(&mut inner.logs_bloom),
        }
    }
    pub fn prev_randao(&self) -> Option<&Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.prev_randao),
            Self::Capella(inner) => Some(&inner.prev_randao),
            Self::Deneb(inner) => Some(&inner.prev_randao),
        }
    }
    pub fn prev_randao_mut(&mut self) -> Option<&mut Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.prev_randao),
            Self::Capella(inner) => Some(&mut inner.prev_randao),
            Self::Deneb(inner) => Some(&mut inner.prev_randao),
        }
    }
    pub fn block_number(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.block_number),
            Self::Capella(inner) => Some(&inner.block_number),
            Self::Deneb(inner) => Some(&inner.block_number),
        }
    }
    pub fn block_number_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.block_number),
            Self::Capella(inner) => Some(&mut inner.block_number),
            Self::Deneb(inner) => Some(&mut inner.block_number),
        }
    }
    pub fn gas_limit(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.gas_limit),
            Self::Capella(inner) => Some(&inner.gas_limit),
            Self::Deneb(inner) => Some(&inner.gas_limit),
        }
    }
    pub fn gas_limit_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.gas_limit),
            Self::Capella(inner) => Some(&mut inner.gas_limit),
            Self::Deneb(inner) => Some(&mut inner.gas_limit),
        }
    }
    pub fn gas_used(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.gas_used),
            Self::Capella(inner) => Some(&inner.gas_used),
            Self::Deneb(inner) => Some(&inner.gas_used),
        }
    }
    pub fn gas_used_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.gas_used),
            Self::Capella(inner) => Some(&mut inner.gas_used),
            Self::Deneb(inner) => Some(&mut inner.gas_used),
        }
    }
    pub fn timestamp(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.timestamp),
            Self::Capella(inner) => Some(&inner.timestamp),
            Self::Deneb(inner) => Some(&inner.timestamp),
        }
    }
    pub fn timestamp_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.timestamp),
            Self::Capella(inner) => Some(&mut inner.timestamp),
            Self::Deneb(inner) => Some(&mut inner.timestamp),
        }
    }
    pub fn extra_data(&self) -> Option<&ByteList<MAX_EXTRA_DATA_BYTES>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.extra_data),
            Self::Capella(inner) => Some(&inner.extra_data),
            Self::Deneb(inner) => Some(&inner.extra_data),
        }
    }
    pub fn extra_data_mut(&mut self) -> Option<&mut ByteList<MAX_EXTRA_DATA_BYTES>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.extra_data),
            Self::Capella(inner) => Some(&mut inner.extra_data),
            Self::Deneb(inner) => Some(&mut inner.extra_data),
        }
    }
    pub fn base_fee_per_gas(&self) -> Option<&U256> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.base_fee_per_gas),
            Self::Capella(inner) => Some(&inner.base_fee_per_gas),
            Self::Deneb(inner) => Some(&inner.base_fee_per_gas),
        }
    }
    pub fn base_fee_per_gas_mut(&mut self) -> Option<&mut U256> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.base_fee_per_gas),
            Self::Capella(inner) => Some(&mut inner.base_fee_per_gas),
            Self::Deneb(inner) => Some(&mut inner.base_fee_per_gas),
        }
    }
    pub fn block_hash(&self) -> Option<&Hash32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.block_hash),
            Self::Capella(inner) => Some(&inner.block_hash),
            Self::Deneb(inner) => Some(&inner.block_hash),
        }
    }
    pub fn block_hash_mut(&mut self) -> Option<&mut Hash32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.block_hash),
            Self::Capella(inner) => Some(&mut inner.block_hash),
            Self::Deneb(inner) => Some(&mut inner.block_hash),
        }
    }
    pub fn transactions(
        &self,
    ) -> Option<&List<Transaction<MAX_BYTES_PER_TRANSACTION>, MAX_TRANSACTIONS_PER_PAYLOAD>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.transactions),
            Self::Capella(inner) => Some(&inner.transactions),
            Self::Deneb(inner) => Some(&inner.transactions),
        }
    }
    pub fn transactions_mut(
        &mut self,
    ) -> Option<&mut List<Transaction<MAX_BYTES_PER_TRANSACTION>, MAX_TRANSACTIONS_PER_PAYLOAD>>
    {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.transactions),
            Self::Capella(inner) => Some(&mut inner.transactions),
            Self::Deneb(inner) => Some(&mut inner.transactions),
        }
    }
    pub fn withdrawals(&self) -> Option<&List<Withdrawal, MAX_WITHDRAWALS_PER_PAYLOAD>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&inner.withdrawals),
            Self::Deneb(inner) => Some(&inner.withdrawals),
        }
    }
    pub fn withdrawals_mut(
        &mut self,
    ) -> Option<&mut List<Withdrawal, MAX_WITHDRAWALS_PER_PAYLOAD>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&mut inner.withdrawals),
            Self::Deneb(inner) => Some(&mut inner.withdrawals),
        }
    }
    pub fn blob_gas_used(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.blob_gas_used),
        }
    }
    pub fn blob_gas_used_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&mut inner.blob_gas_used),
        }
    }
    pub fn excess_blob_gas(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.excess_blob_gas),
        }
    }
    pub fn excess_blob_gas_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&mut inner.excess_blob_gas),
        }
    }
}
impl<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    > serde::Serialize
    for ExecutionPayload<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Bellatrix(inner) => <_ as serde::Serialize>::serialize(inner, serializer),
            Self::Capella(inner) => <_ as serde::Serialize>::serialize(inner, serializer),
            Self::Deneb(inner) => <_ as serde::Serialize>::serialize(inner, serializer),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionPayloadRef<
    'a,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
> {
    Bellatrix(
        &'a bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ),
    Capella(
        &'a capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ),
    Deneb(
        &'a deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ),
}
impl<
        'a,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    ExecutionPayloadRef<
        'a,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella(
        &self,
    ) -> Option<
        &capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb(
        &self,
    ) -> Option<
        &deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
        }
    }
    pub fn parent_hash(&self) -> Option<&Hash32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.parent_hash),
            Self::Capella(inner) => Some(&inner.parent_hash),
            Self::Deneb(inner) => Some(&inner.parent_hash),
        }
    }
    pub fn fee_recipient(&self) -> Option<&ExecutionAddress> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.fee_recipient),
            Self::Capella(inner) => Some(&inner.fee_recipient),
            Self::Deneb(inner) => Some(&inner.fee_recipient),
        }
    }
    pub fn state_root(&self) -> Option<&Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.state_root),
            Self::Capella(inner) => Some(&inner.state_root),
            Self::Deneb(inner) => Some(&inner.state_root),
        }
    }
    pub fn receipts_root(&self) -> Option<&Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.receipts_root),
            Self::Capella(inner) => Some(&inner.receipts_root),
            Self::Deneb(inner) => Some(&inner.receipts_root),
        }
    }
    pub fn logs_bloom(&self) -> Option<&ByteVector<BYTES_PER_LOGS_BLOOM>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.logs_bloom),
            Self::Capella(inner) => Some(&inner.logs_bloom),
            Self::Deneb(inner) => Some(&inner.logs_bloom),
        }
    }
    pub fn prev_randao(&self) -> Option<&Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.prev_randao),
            Self::Capella(inner) => Some(&inner.prev_randao),
            Self::Deneb(inner) => Some(&inner.prev_randao),
        }
    }
    pub fn block_number(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.block_number),
            Self::Capella(inner) => Some(&inner.block_number),
            Self::Deneb(inner) => Some(&inner.block_number),
        }
    }
    pub fn gas_limit(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.gas_limit),
            Self::Capella(inner) => Some(&inner.gas_limit),
            Self::Deneb(inner) => Some(&inner.gas_limit),
        }
    }
    pub fn gas_used(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.gas_used),
            Self::Capella(inner) => Some(&inner.gas_used),
            Self::Deneb(inner) => Some(&inner.gas_used),
        }
    }
    pub fn timestamp(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.timestamp),
            Self::Capella(inner) => Some(&inner.timestamp),
            Self::Deneb(inner) => Some(&inner.timestamp),
        }
    }
    pub fn extra_data(&self) -> Option<&ByteList<MAX_EXTRA_DATA_BYTES>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.extra_data),
            Self::Capella(inner) => Some(&inner.extra_data),
            Self::Deneb(inner) => Some(&inner.extra_data),
        }
    }
    pub fn base_fee_per_gas(&self) -> Option<&U256> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.base_fee_per_gas),
            Self::Capella(inner) => Some(&inner.base_fee_per_gas),
            Self::Deneb(inner) => Some(&inner.base_fee_per_gas),
        }
    }
    pub fn block_hash(&self) -> Option<&Hash32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.block_hash),
            Self::Capella(inner) => Some(&inner.block_hash),
            Self::Deneb(inner) => Some(&inner.block_hash),
        }
    }
    pub fn transactions(
        &self,
    ) -> Option<&List<Transaction<MAX_BYTES_PER_TRANSACTION>, MAX_TRANSACTIONS_PER_PAYLOAD>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.transactions),
            Self::Capella(inner) => Some(&inner.transactions),
            Self::Deneb(inner) => Some(&inner.transactions),
        }
    }
    pub fn withdrawals(&self) -> Option<&List<Withdrawal, MAX_WITHDRAWALS_PER_PAYLOAD>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&inner.withdrawals),
            Self::Deneb(inner) => Some(&inner.withdrawals),
        }
    }
    pub fn blob_gas_used(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.blob_gas_used),
        }
    }
    pub fn excess_blob_gas(&self) -> Option<&u64> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.excess_blob_gas),
        }
    }
}
impl<
        'a,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    From<
        &'a bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    >
    for ExecutionPayloadRef<
        'a,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ) -> Self {
        Self::Bellatrix(value)
    }
}
impl<
        'a,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    From<
        &'a capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    >
    for ExecutionPayloadRef<
        'a,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ) -> Self {
        Self::Capella(value)
    }
}
impl<
        'a,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    From<
        &'a deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    >
    for ExecutionPayloadRef<
        'a,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ) -> Self {
        Self::Deneb(value)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionPayloadRefMut<
    'a,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
> {
    Bellatrix(
        &'a mut bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ),
    Capella(
        &'a mut capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ),
    Deneb(
        &'a mut deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ),
}
impl<
        'a,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    ExecutionPayloadRefMut<
        'a,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    pub fn bellatrix_mut(
        &mut self,
    ) -> Option<
        &mut bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella_mut(
        &mut self,
    ) -> Option<
        &mut capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb_mut(
        &mut self,
    ) -> Option<
        &mut deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
        }
    }
    pub fn parent_hash_mut(&mut self) -> Option<&mut Hash32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.parent_hash),
            Self::Capella(inner) => Some(&mut inner.parent_hash),
            Self::Deneb(inner) => Some(&mut inner.parent_hash),
        }
    }
    pub fn fee_recipient_mut(&mut self) -> Option<&mut ExecutionAddress> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.fee_recipient),
            Self::Capella(inner) => Some(&mut inner.fee_recipient),
            Self::Deneb(inner) => Some(&mut inner.fee_recipient),
        }
    }
    pub fn state_root_mut(&mut self) -> Option<&mut Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.state_root),
            Self::Capella(inner) => Some(&mut inner.state_root),
            Self::Deneb(inner) => Some(&mut inner.state_root),
        }
    }
    pub fn receipts_root_mut(&mut self) -> Option<&mut Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.receipts_root),
            Self::Capella(inner) => Some(&mut inner.receipts_root),
            Self::Deneb(inner) => Some(&mut inner.receipts_root),
        }
    }
    pub fn logs_bloom_mut(&mut self) -> Option<&mut ByteVector<BYTES_PER_LOGS_BLOOM>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.logs_bloom),
            Self::Capella(inner) => Some(&mut inner.logs_bloom),
            Self::Deneb(inner) => Some(&mut inner.logs_bloom),
        }
    }
    pub fn prev_randao_mut(&mut self) -> Option<&mut Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.prev_randao),
            Self::Capella(inner) => Some(&mut inner.prev_randao),
            Self::Deneb(inner) => Some(&mut inner.prev_randao),
        }
    }
    pub fn block_number_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.block_number),
            Self::Capella(inner) => Some(&mut inner.block_number),
            Self::Deneb(inner) => Some(&mut inner.block_number),
        }
    }
    pub fn gas_limit_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.gas_limit),
            Self::Capella(inner) => Some(&mut inner.gas_limit),
            Self::Deneb(inner) => Some(&mut inner.gas_limit),
        }
    }
    pub fn gas_used_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.gas_used),
            Self::Capella(inner) => Some(&mut inner.gas_used),
            Self::Deneb(inner) => Some(&mut inner.gas_used),
        }
    }
    pub fn timestamp_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.timestamp),
            Self::Capella(inner) => Some(&mut inner.timestamp),
            Self::Deneb(inner) => Some(&mut inner.timestamp),
        }
    }
    pub fn extra_data_mut(&mut self) -> Option<&mut ByteList<MAX_EXTRA_DATA_BYTES>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.extra_data),
            Self::Capella(inner) => Some(&mut inner.extra_data),
            Self::Deneb(inner) => Some(&mut inner.extra_data),
        }
    }
    pub fn base_fee_per_gas_mut(&mut self) -> Option<&mut U256> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.base_fee_per_gas),
            Self::Capella(inner) => Some(&mut inner.base_fee_per_gas),
            Self::Deneb(inner) => Some(&mut inner.base_fee_per_gas),
        }
    }
    pub fn block_hash_mut(&mut self) -> Option<&mut Hash32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.block_hash),
            Self::Capella(inner) => Some(&mut inner.block_hash),
            Self::Deneb(inner) => Some(&mut inner.block_hash),
        }
    }
    pub fn transactions_mut(
        &mut self,
    ) -> Option<&mut List<Transaction<MAX_BYTES_PER_TRANSACTION>, MAX_TRANSACTIONS_PER_PAYLOAD>>
    {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.transactions),
            Self::Capella(inner) => Some(&mut inner.transactions),
            Self::Deneb(inner) => Some(&mut inner.transactions),
        }
    }
    pub fn withdrawals_mut(
        &mut self,
    ) -> Option<&mut List<Withdrawal, MAX_WITHDRAWALS_PER_PAYLOAD>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&mut inner.withdrawals),
            Self::Deneb(inner) => Some(&mut inner.withdrawals),
        }
    }
    pub fn blob_gas_used_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&mut inner.blob_gas_used),
        }
    }
    pub fn excess_blob_gas_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&mut inner.excess_blob_gas),
        }
    }
}
impl<
        'a,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    From<
        &'a mut bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    >
    for ExecutionPayloadRefMut<
        'a,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a mut bellatrix::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ) -> Self {
        Self::Bellatrix(value)
    }
}
impl<
        'a,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    From<
        &'a mut capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    >
    for ExecutionPayloadRefMut<
        'a,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a mut capella::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ) -> Self {
        Self::Capella(value)
    }
}
impl<
        'a,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    >
    From<
        &'a mut deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    >
    for ExecutionPayloadRefMut<
        'a,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a mut deneb::ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    ) -> Self {
        Self::Deneb(value)
    }
}
