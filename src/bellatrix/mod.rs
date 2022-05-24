//! This module provides an implementation of the `bellatrix` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
mod blinded_beacon_block;
mod execution;
mod presets;

pub use beacon_block::*;
pub use beacon_state::*;
pub use blinded_beacon_block::*;
pub use execution::*;

pub mod mainnet {
    pub use super::presets::mainnet::*;
}
