mod context;
mod error;

pub use context::Context;
pub use error::*;

pub enum Validation {
    Enabled,
    Disabled,
}
