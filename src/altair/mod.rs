//! This module provides an implementation of the `altair` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
pub mod light_client;
mod presets;
mod sync;
mod validator;

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub mod minimal {}

pub use beacon_block::*;
pub use beacon_state::*;
pub use sync::*;
pub use validator::*;
