//! This module provides an implementation of the `altair` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
pub mod beacon_block;
pub mod beacon_state;
pub mod light_client;
pub mod presets;
pub(crate) mod sync;
mod validator;

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub mod minimal {}
