//! This module provides an implementation of the `phase0` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
mod fork;
mod operations;
mod presets;
mod signing;
mod state_transition;
mod validator;

pub use beacon_block::*;
pub use beacon_state::*;
pub use fork::*;
pub use operations::*;
pub use signing::*;
pub use state_transition::*;
pub use validator::*;

pub const BASE_REWARDS_PER_EPOCH: u64 = 4;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: usize = 2usize.pow(5);
pub const JUSTIFICATION_BITS_LENGTH: usize = 4;

pub mod mainnet {
    pub use super::presets::mainnet::*;
    pub use super::Context;

    pub fn apply_block(
        state: &mut BeaconState,
        signed_block: &mut SignedBeaconBlock,
        context: &Context,
    ) -> Result<(), super::Error> {
        super::state_transition(state, signed_block, Some(true), context)
    }
}

pub mod minimal {}
