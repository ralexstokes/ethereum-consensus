//! This module provides an implementation of the `phase0` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
mod fork;
mod operations;
mod presets;
pub mod state_transition;
mod validator;

pub use beacon_block::*;
pub use beacon_state::*;
pub use fork::*;
pub use operations::*;
pub use presets::Preset;
pub use state_transition::{
    block_processing::*, epoch_processing::*, genesis, helpers::*, slot_processing::*, *,
};
pub use validator::*;

pub const BASE_REWARDS_PER_EPOCH: u64 = 4;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: usize = 2usize.pow(5);
pub const JUSTIFICATION_BITS_LENGTH: usize = 4;

pub mod mainnet {
    pub use super::presets::mainnet::*;

    pub fn genesis_state() -> BeaconState {
        // TODO return actual genesis state
        BeaconState::default()
    }
}

pub mod minimal {}
