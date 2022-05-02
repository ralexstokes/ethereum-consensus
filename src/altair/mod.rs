//! This module provides an implementation of the `altair` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
pub mod light_client;
mod presets;
mod sync;

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub mod minimal {}

pub type BeaconBlockHeader = crate::phase0::mainnet::BeaconBlockHeader;

pub const SYNC_COMMITTEE_SIZE: usize = 512;
