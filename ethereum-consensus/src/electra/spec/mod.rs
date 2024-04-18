pub use crate::electra::{
    beacon_block::{BeaconBlock, BeaconBlockBody, SignedBeaconBlock},
    beacon_state::{BeaconState, DepositReceipt, ExecutionLayerWithdrawalRequest},
    constants::UNSET_DEPOSIT_RECEIPTS_START_INDEX,
    execution_payload::{ExecutionPayload, ExecutionPayloadHeader},
    helpers::get_committee_indices,
    operations::{Attestation, IndexedAttestation},
};
