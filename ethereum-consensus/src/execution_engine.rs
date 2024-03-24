use crate::state_transition::Result;

/// `ExecutionEngine` abstracts over the interface between consensus and execution client.
pub trait ExecutionEngine {
    type NewPayloadRequest;

    /// Verify the new payload and associated data contained in `Self::NewPayloadRequest`.
    /// Either return `Err` describing a failure or `Ok(())` if validation succeeds.
    fn verify_and_notify_new_payload(
        &self,
        new_payload_request: &Self::NewPayloadRequest,
    ) -> Result<()>;
}

/// "no-op" implementation for convenience.
/// Always succeeds validation.
impl ExecutionEngine for () {
    type NewPayloadRequest = ();

    fn verify_and_notify_new_payload(&self, _: &Self::NewPayloadRequest) -> Result<()> {
        Ok(())
    }
}
