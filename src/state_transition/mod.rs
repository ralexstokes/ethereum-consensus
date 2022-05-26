mod context;
mod error;

pub use context::*;
pub use error::*;

pub enum Validation {
    Enabled,
    Disabled,
}
