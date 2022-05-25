//! This module provides an implementation of the `altair` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
pub mod block_processing;
pub mod epoch_processing;
pub mod helpers;
pub mod light_client;
mod presets;
mod sync;
mod validator;

pub mod state_transition;
pub use state_transition::{
    block_processing::*, epoch_processing::*, genesis, helpers::*, slot_processing::*, *,
};

pub use beacon_block::*;
pub use beacon_state::*;
pub use presets::Preset;
pub use sync::*;
pub use validator::*;

pub use crate::phase0::{
    Attestation, AttestationData, AttesterSlashing, BeaconBlockHeader, Checkpoint, Deposit,
    DepositData, DepositMessage, Eth1Data, Fork, ForkData, HistoricalBatchAccumulator,
    IndexedAttestation, ProposerSlashing, SignedVoluntaryExit, Validator, BASE_REWARDS_PER_EPOCH,
    DEPOSIT_CONTRACT_TREE_DEPTH, JUSTIFICATION_BITS_LENGTH,
};

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub mod minimal {}
