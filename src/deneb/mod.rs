//! This module provides an implementation of the `deneb` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
mod blinded_beacon_block;
mod blinded_blob_sidecar;
mod blob_sidecar;
mod execution;
mod presets;

pub use beacon_block::*;
pub use beacon_state::*;
pub use blinded_beacon_block::*;
pub use blinded_blob_sidecar::*;
pub use blob_sidecar::*;
pub use execution::*;
pub use presets::Preset;

pub use crate::altair::{
    SyncAggregate, SyncAggregatorSelectionData, SyncCommittee, PARTICIPATION_FLAG_WEIGHTS,
    PROPOSER_WEIGHT, SYNC_REWARD_WEIGHT, TIMELY_HEAD_FLAG_INDEX, TIMELY_SOURCE_FLAG_INDEX,
    TIMELY_TARGET_FLAG_INDEX, WEIGHT_DENOMINATOR,
};
pub use crate::bellatrix::Transaction;
pub use crate::capella::{BlsToExecutionChange, SignedBlsToExecutionChange, Withdrawal};
pub use crate::phase0::{
    Attestation, AttestationData, AttesterSlashing, BeaconBlockHeader, Checkpoint, Deposit,
    DepositData, DepositMessage, Eth1Block, Eth1Data, Fork, ForkData, HistoricalSummary,
    IndexedAttestation, ProposerSlashing, SignedBeaconBlockHeader, SignedVoluntaryExit,
    SigningData, Validator, VoluntaryExit, BASE_REWARDS_PER_EPOCH, DEPOSIT_CONTRACT_TREE_DEPTH,
    JUSTIFICATION_BITS_LENGTH,
};

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub mod minimal {
    pub use super::presets::minimal::*;
}

pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
