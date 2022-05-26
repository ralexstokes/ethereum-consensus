//! WARNING: This file was derived by the `gen-spec` utility. DO NOT EDIT MANUALLY.
pub mod block_processing;
pub mod epoch_processing;
pub mod genesis;
pub mod helpers;
pub mod slot_processing;

pub use crate::bellatrix::state_transition_bellatrix::state_transition;
use ssz_rs::prelude::*;
