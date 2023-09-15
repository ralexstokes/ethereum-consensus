// use crate::{
//     capella::execution_payload::ExecutionPayload,
//     execution,
//     primitives::{Bytes32, ExecutionAddress, Hash32},
// };
// use std::marker::PhantomData;

// pub struct NewPayloadRequest<E> {
//     pub execution_payload: E,
// }

// pub struct GetPayloadResponse<E> {
//     pub execution_payload: E,
// }

// pub struct PayloadAttributes {
//     timestamp: u64,
//     prev_randao: Bytes32,
//     suggested_fee_recipient: ExecutionAddress,
// }

// pub struct ExecutionEngine<E>(PhantomData<E>);

// impl<E> ExecutionEngine<E> {
//     pub fn notify_new_payload(&self, execution_payload: &E) -> Result<(), execution::Error> {
//         Ok(())
//     }

//     pub fn is_valid_block_hash(&self, execution_payload: &E) -> Result<(), execution::Error> {
//         Ok(())
//     }
// }

// impl<E> execution::ExecutionEngine for ExecutionEngine<E> {
//     type NewPayloadRequest = NewPayloadRequest<E>;
//     type GetPayloadResponse = GetPayloadResponse<E>;
//     type PayloadAttributes = PayloadAttributes;

//     fn verify_and_notify_new_payload(
//         &self,
//         new_payload_request: &Self::NewPayloadRequest,
//     ) -> Result<(), execution::Error> { if
//       !self.is_valid_block_hash(&new_payload_request.execution_payload) { return
//       Err(execution::Error::InvalidBlockHash.into()) }
//       self.notify_new_payload(&new_payload_request.execution_payload)
//     }

//     fn notify_forkchoice_updated(
//         &self,
//         head_block_hash: Hash32,
//         safe_block_hash: Hash32,
//         finalized_block_hash: Hash32,
//         payload_attributes: Option<Self::PayloadAttributes>,
//     ) -> Result<Option<execution::PayloadId>, execution::Error> { Ok(None)
//     }

//     fn get_payload(&self, payload_id: execution::PayloadId) -> Self::GetPayloadResponse {
//         let execution_payload = Self::ExecutionPayload::default();
//         GetPayloadResponse { execution_payload }.into()
//     }
// }
