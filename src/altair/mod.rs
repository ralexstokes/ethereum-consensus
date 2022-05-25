//! This module provides an implementation of the `altair` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
// mod block_processing;
pub mod block_processing_altair;
// mod epoch_processing;
pub mod epoch_processing_altair;
// mod helpers;
pub mod genesis_altair;
pub mod helpers_altair;
pub mod light_client;
mod presets;
mod sync;
mod validator;
// mod slot_processing;
// mod state_transition;

pub use crate::phase0::{
    Attestation, AttestationData, AttesterSlashing, BeaconBlockHeader, Checkpoint, Deposit,
    DepositMessage, Eth1Data, Fork, ForkData, HistoricalBatchAccumulator, IndexedAttestation,
    PendingAttestation, ProposerSlashing, SignedVoluntaryExit, SigningData, Validator,
    BASE_REWARDS_PER_EPOCH, DEPOSIT_CONTRACT_TREE_DEPTH, JUSTIFICATION_BITS_LENGTH,
};

pub use beacon_block::*;
pub use beacon_state::*;
// pub use block_processing::*;
// pub use epoch_processing::*;
// pub use helpers::*;
pub use presets::Preset;
// pub use slot_processing::*;
// pub use state_transition::*;
pub use sync::*;
pub use validator::*;

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub mod minimal {}
