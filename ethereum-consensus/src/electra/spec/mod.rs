pub use crate::electra::{
    beacon_block::{BeaconBlock, BeaconBlockBody, SignedBeaconBlock},
    beacon_state::BeaconState,
    constants::UNSET_DEPOSIT_RECEIPTS_START_INDEX,
    deposit_receipt::DepositReceipt,
    execution_layer_exit::ExecutionLayerExit,
    execution_payload::{ExecutionPayload, ExecutionPayloadHeader},
    operations::Attestation,
};
