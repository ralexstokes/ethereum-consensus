mod beacon_block;
mod beacon_state;
mod context;
pub mod error;
mod execution_engine;
mod executor;
mod presets;

pub use beacon_block::*;
pub use beacon_state::*;
pub use context::*;
pub use error::*;
pub use execution_engine::*;
pub use executor::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy)]
pub enum Validation {
    Enabled,
    Disabled,
}

pub mod mainnet {
    pub use super::presets::mainnet::*;
}

pub mod minimal {
    pub use super::presets::minimal::*;
}
