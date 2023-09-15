use crate::{bellatrix::ExecutionPayload, execution::Error};

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

impl ExecutionEngine for () {
    fn is_valid_block_hash(&self, payload: &ExecutionPayload) -> Result<(), Error> {
        Ok(())
    }

    fn notify_new_payload(&self, payload: &ExecutionPayload) -> Result<(), Error> {
        Ok(())
    }

    fn verify_and_notify_new_payload(
        &self,
        new_payload_request: &NewPayloadRequest,
    ) -> Result<(), execution::Error> {
        self.is_valid_block_hash(&new_payload_request.execution_payload())?;
        self.notify_new_payload(&new_payload_request.execution_payload())
    }
}

pub trait ExecutionEngine {
    fn is_valid_block_hash(&self, payload: &ExecutionPayload) -> Result<(), Error>;
    fn notify_new_payload(&self, payload: &ExecutionPayload) -> Result<(), Error>;

    fn verify_and_notify_new_payload(
        &self,
        new_payload_request: &NewPayloadRequest,
    ) -> Result<(), Error> {
        self.is_valid_block_hash(&new_payload_request.execution_payload())?;
        self.notify_new_payload(&new_payload_request.execution_payload())
    }
}
