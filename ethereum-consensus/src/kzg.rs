use crate::{primitives::Bytes32, ssz::ByteVector};
use ssz_rs::prelude::*;

pub const KZG_COMMITMENT_BYTES_LEN: usize = 48;
pub const KZG_PROOF_BYTES_LEN: usize = 48;

pub type VersionedHash = Bytes32;
pub type KzgCommitment = ByteVector<KZG_COMMITMENT_BYTES_LEN>;
pub type KzgProof = ByteVector<KZG_PROOF_BYTES_LEN>;
