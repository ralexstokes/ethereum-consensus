//! This module defines different implementations depending on the
//! requirement to verify any of the underlying cryptography or simply
//! treat the data for each type as opaque bytes.
//! This option exists to facilitate some testing which takes the latter view.

#[cfg(feature = "bls")]
mod with_bls;
#[cfg(not(feature = "bls"))]
mod without_bls;

#[cfg(feature = "bls")]
pub use with_bls::*;
#[cfg(not(feature = "bls"))]
pub use without_bls::*;

use crate::primitives::Bytes32;
use sha2::{digest::FixedOutput, Digest, Sha256};

pub fn hash<D: AsRef<[u8]>>(data: D) -> Bytes32 {
    let mut result = vec![0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize_into(result.as_mut_slice().into());
    Bytes32::try_from(result.as_ref()).expect("correct input")
}
