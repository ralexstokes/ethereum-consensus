mod bundler;
mod command;
mod decode;
mod encode;
mod framing;

pub use command::Command;
use ethereum_consensus::{deneb as spec, Error as ConsensusError};
use thiserror::Error;

pub(crate) const BYTES_PER_BLOB: usize = spec::mainnet::BYTES_PER_BLOB;
pub(crate) const BYTES_PER_FIELD_ELEMENT: usize =
    spec::polynomial_commitments::BYTES_PER_FIELD_ELEMENT;
// Number of bits in a valid field element.
pub(crate) const BITS_PER_FIELD_ELEMENT: usize = 254;

pub(crate) type Blob = spec::mainnet::Blob;

#[derive(Debug, Error)]
pub enum Error {
    #[error("data is not a field element")]
    InvalidFieldElement,
    #[error("sized framing was requested but decoded stream is not large enough for frame header")]
    ExpectedHeaderForSizedFraming,
    #[error("sized framing was requested but the encoded version is not supported")]
    UnsupportedSizedFramingVersion,
    #[error("sized framing was requested but the encoded size exceeds the maximum possible size")]
    InvalidPayloadSize,
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Consensus(#[from] ConsensusError),
}
