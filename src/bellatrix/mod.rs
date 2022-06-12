//! This module provides an implementation of the `bellatrix` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
mod blinded_beacon_block;
pub mod block_processing;
pub mod epoch_processing;
mod execution;
pub mod helpers;
mod presets;
pub mod state_transition_bellatrix;

pub mod state_transition;
pub use state_transition::{
    block_processing::*, epoch_processing::*, genesis, helpers::*, slot_processing::*, *,
};

pub use beacon_block::*;
pub use beacon_state::*;
pub use blinded_beacon_block::*;
pub use execution::*;
pub use presets::Preset;
pub use state_transition_bellatrix::*;

pub use crate::altair::{
    SyncAggregate, PROPOSER_WEIGHT, TIMELY_TARGET_FLAG_INDEX, WEIGHT_DENOMINATOR,
};
pub use crate::phase0::{
    Attestation, AttestationData, AttesterSlashing, BeaconBlockHeader, Checkpoint, Deposit,
    DepositData, DepositMessage, Eth1Data, Fork, ForkData, HistoricalBatchAccumulator,
    IndexedAttestation, ProposerSlashing, SignedVoluntaryExit, Validator, BASE_REWARDS_PER_EPOCH,
    DEPOSIT_CONTRACT_TREE_DEPTH, JUSTIFICATION_BITS_LENGTH,
};
pub use crate::primitives::ParticipationFlags;

pub mod mainnet {
    pub use super::presets::mainnet::*;
}
