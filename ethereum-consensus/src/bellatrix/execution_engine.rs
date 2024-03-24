use crate::{
    bellatrix::execution_payload::ExecutionPayload, error::ExecutionEngineError,
    execution_engine::ExecutionEngine, state_transition::Result,
};

// The `DefaultExecutionEngine` performs no operations and validation
// is determined by `execution_is_valid`.
#[derive(Debug)]
pub struct DefaultExecutionEngine<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
> {
    execution_is_valid: bool,
}

impl<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    > Default
    for DefaultExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >
{
    fn default() -> Self {
        Self { execution_is_valid: true }
    }
}

impl<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    >
    DefaultExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >
{
    pub fn new(execution_is_valid: bool) -> Self {
        Self { execution_is_valid }
    }

    fn is_valid_block_hash(
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

    fn notify_new_payload(
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
    > ExecutionEngine
    for DefaultExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >
{
    type NewPayloadRequest = ExecutionPayload<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >;
    fn verify_and_notify_new_payload(
        &self,
        new_payload_request: &Self::NewPayloadRequest,
    ) -> Result<()> {
        self.is_valid_block_hash(new_payload_request)?;
        self.notify_new_payload(new_payload_request)
    }
}
