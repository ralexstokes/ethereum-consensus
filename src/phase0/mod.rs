//! This module provides an implementation of the `phase0` fork
//! of the consensus spec. The primary entrypoints should be one of
//! the "presets" like `mainnet` or `minimal`.
mod beacon_block;
mod beacon_state;
mod block_processing;
mod epoch_processing;
mod fork;
pub mod genesis;
mod helpers;
mod operations;
mod presets;
mod slot_processing;
mod state_transition;
mod validator;

pub use beacon_block::*;
pub use beacon_state::*;

pub use crate::signing::SigningData;
pub use block_processing::*;
pub use epoch_processing::*;
pub use fork::*;
pub use helpers::*;
pub use operations::*;
pub use presets::Preset;
pub use slot_processing::*;
pub use state_transition::*;
pub use validator::*;

pub const BASE_REWARDS_PER_EPOCH: u64 = 4;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: usize = 2usize.pow(5);
pub const JUSTIFICATION_BITS_LENGTH: usize = 4;

pub use presets::mainnet;
pub use presets::minimal;
