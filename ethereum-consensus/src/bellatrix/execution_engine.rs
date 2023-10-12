use crate::{
    bellatrix::execution_payload::ExecutionPayload,
    error::ExecutionEngineError,
    state_transition::{self, Result},
};

pub struct NewPayloadRequest<
    'a,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    pub  &'a ExecutionPayload<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
);

pub trait ExecutionEngine<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>
{
    fn verify_and_notify_new_payload(
        &self,
        new_payload_request: &NewPayloadRequest<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ) -> Result<()>;
}

// The `DefaultExecutionEngine` performs no operations and validation
// is determined by `execution_is_valid`.
#[derive(Debug)]
pub struct DefaultExecutionEngine {
    execution_is_valid: bool,
}

impl Default for DefaultExecutionEngine {
    fn default() -> Self {
        Self { execution_is_valid: true }
    }
}

impl DefaultExecutionEngine {
    pub fn new(execution_is_valid: bool) -> Self {
        Self { execution_is_valid }
    }

    fn is_valid_block_hash<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    >(
        &self,
        _payload: &ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ) -> Result<()> {
        if !self.execution_is_valid {
            Err(ExecutionEngineError::InvalidBlockHash.into())
        } else {
            Ok(())
        }
    }

    fn notify_new_payload<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    >(
        &self,
        _payload: &ExecutionPayload<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ) -> Result<()> {
        if !self.execution_is_valid {
            Err(ExecutionEngineError::InvalidPayload.into())
        } else {
            Ok(())
        }
    }
}

impl<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    >
    ExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    > for DefaultExecutionEngine
{
    fn verify_and_notify_new_payload(
        &self,
        new_payload_request: &NewPayloadRequest<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    ) -> Result<()> {
        self.is_valid_block_hash(new_payload_request.0)?;
        self.notify_new_payload(new_payload_request.0)
    }
}

impl<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        B: ExecutionEngine<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
    > From<B>
    for state_transition::ExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        B,
    >
{
    fn from(value: B) -> Self {
        Self::Bellatrix(value)
    }
}
