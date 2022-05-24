/// This module provides utilities for executing a state transition throughout a
/// chain's lifetime, including across multiple forks.
/// At a high-level, the common functionality to the state transition across
/// all forks can be found in this module and its children modules.
/// Fork-specific functionality can be found in the fork-specific modules
/// defined as a sibling to this one.
pub mod block_processing;
pub mod context;
pub mod epoch_processing;
mod error;
mod executor;
pub mod helpers;
pub mod slot_processing;
pub mod spec;
mod types;

pub use context::Context;
pub use error::*;
pub use executor::Executor;
pub use helpers::*;
pub use types::*;
