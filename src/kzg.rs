use crate::bytes::write_bytes_to_lower_hex;
use crate::primitives::Bytes32;
use crate::serde::{try_bytes_from_hex_str, HexError};
use crate::ssz::ByteVector;
use core::fmt;
use ssz_rs::prelude::*;
use std::ops::{Deref, DerefMut};
use thiserror::Error;

const KZG_COMMITMENT_BYTES_LEN: usize = 48;
const KZG_PROOF_BYTES_LEN: usize = 48;

pub type VersionedHash = Bytes32;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "serde")]
    #[error("error deserializing hex-encoded input: {0}")]
    Hex(#[from] HexError),
    #[error(
        "invalid length of encoding: expected {expected} bytes but only provided {provided} bytes"
    )]
    EncodingError { provided: usize, expected: usize },
}

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String", try_from = "String"))]
pub struct KzgCommitment(ByteVector<KZG_COMMITMENT_BYTES_LEN>);

impl fmt::LowerHex for KzgCommitment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_bytes_to_lower_hex(f, self.as_ref())
    }
}

impl fmt::Display for KzgCommitment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl Deref for KzgCommitment {
    type Target = ByteVector<KZG_COMMITMENT_BYTES_LEN>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KzgCommitment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "serde")]
impl From<KzgCommitment> for String {
    fn from(kzg_commitment: KzgCommitment) -> Self {
        format!("{kzg_commitment}")
    }
}

#[cfg(feature = "serde")]
impl TryFrom<String> for KzgCommitment {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Error> {
        let encoding = try_bytes_from_hex_str(&s)?;
        Self::try_from(encoding.as_ref())
    }
}

impl TryFrom<&[u8]> for KzgCommitment {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Error> {
        let inner = ByteVector::try_from(data).map_err(|_| Error::EncodingError {
            provided: data.len(),
            expected: KZG_COMMITMENT_BYTES_LEN,
        })?;
        Ok(Self(inner))
    }
}

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String", try_from = "String"))]
pub struct KzgProof(ByteVector<KZG_PROOF_BYTES_LEN>);

impl fmt::LowerHex for KzgProof {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_bytes_to_lower_hex(f, self.as_ref())
    }
}

impl fmt::Display for KzgProof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl Deref for KzgProof {
    type Target = ByteVector<KZG_PROOF_BYTES_LEN>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KzgProof {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "serde")]
impl From<KzgProof> for String {
    fn from(kzg_proof: KzgProof) -> Self {
        format!("{kzg_proof}")
    }
}

#[cfg(feature = "serde")]
impl TryFrom<String> for KzgProof {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Error> {
        let encoding = try_bytes_from_hex_str(&s)?;
        Self::try_from(encoding.as_ref())
    }
}

impl TryFrom<&[u8]> for KzgProof {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Error> {
        let inner = ByteVector::try_from(data).map_err(|_| Error::EncodingError {
            provided: data.len(),
            expected: KZG_PROOF_BYTES_LEN,
        })?;
        Ok(Self(inner))
    }
}
