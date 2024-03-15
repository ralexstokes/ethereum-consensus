mod command;
mod decode;
mod encode;
mod framing;

use crate::bls::MODULUS;
pub use command::Command;
use ethereum_consensus::deneb as spec;
use ruint::aliases::U256;
use thiserror::Error;

pub(crate) const BYTES_PER_BLOB: usize = spec::mainnet::BYTES_PER_BLOB;
pub(crate) const BYTES_PER_FIELD_ELEMENT: usize =
    spec::polynomial_commitments::BYTES_PER_FIELD_ELEMENT;
// Number of bits in a valid field element.
pub(crate) const BITS_PER_FIELD_ELEMENT: usize = 254;

pub(crate) type Blob = spec::Blob<BYTES_PER_BLOB>;
pub(crate) type BitSlice = bitvec::slice::BitSlice<u8, bitvec::prelude::Lsb0>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("data is not a field element")]
    ExceedsField,
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("requested framing mode `{0}` is not supported")]
    InvalidFrameMode(String),
    #[error("requested framing for `{0}` bytes but only up to 2^32 bytes is supported")]
    ExceedsMaxFrameSize(usize),
}

pub(crate) fn verify_field_element_bytes(field_element: &[u8]) -> Result<(), Error> {
    let x = U256::try_from_le_slice(field_element).ok_or(Error::ExceedsField)?;
    if x >= MODULUS {
        Err(Error::ExceedsField)
    } else {
        Ok(())
    }
}
