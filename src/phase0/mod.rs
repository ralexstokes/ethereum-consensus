//! This module provides an implementation of the `phase0` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
pub mod configs;
mod fork;
mod operations;
mod presets;
mod state_transition;
mod validator;

pub use presets::{mainnet, minimal};

pub const BASE_REWARDS_PER_EPOCH: usize = 4;
