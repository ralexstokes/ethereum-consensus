use crate::{primitives::Bytes32, ssz::prelude::ByteVector};

pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const KZG_COMMITMENT_BYTES_LEN: usize = 48;
pub const KZG_PROOF_BYTES_LEN: usize = 48;

pub type VersionedHash = Bytes32;
pub type KzgCommitment = ByteVector<KZG_COMMITMENT_BYTES_LEN>;
pub type KzgProof = ByteVector<KZG_PROOF_BYTES_LEN>;
