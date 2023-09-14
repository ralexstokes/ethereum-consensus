#![allow(unused)]
use crate::{crypto::hash, primitives::Bytes32, ssz::prelude::ByteVector};
use alloy_primitives::{uint, U256};
use blst::min_pk::PublicKey;
use c_kzg::{Error, KzgSettings};
use ssz_rs::prelude::*;
use std::ops::Deref;

pub const BLS_MODULUS: U256 =
    uint!(52435875175126190479447740508185965837690552500527637822603658699938581184513_U256);
pub const BYTES_PER_BLOB: usize = 32 * 4096;
pub const BYTES_PER_COMMITMENT: usize = 48;
pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BYTES_PER_PROOF: usize = 48;
pub const KZG_COMMITMENT_BYTES_LEN: usize = 48;
pub const KZG_PROOF_BYTES_LEN: usize = 48;

pub type VersionedHash = Bytes32;
pub type BLSFieldElement = U256;
pub type Polynomial = Vec<BLSFieldElement>; // Should this polynomial type be an array?

const fn create_g1_point_at_infinity() -> [u8; 48] {
    let mut arr: [u8; 48] = [0; 48];
    arr[0] = 0xc0;
    arr
}

/// TODO:  Lean on C-KZG library to implement specs.

pub struct Blob(ByteVector<BYTES_PER_BLOB>);

#[derive(SimpleSerialize, Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KzgCommitment(ByteVector<BYTES_PER_COMMITMENT>);

impl Deref for KzgCommitment {
    type Target = ByteVector<BYTES_PER_COMMITMENT>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(SimpleSerialize, Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KzgProof(ByteVector<BYTES_PER_PROOF>);

/// Converts blob to kzg_commitment by using multicalar multiplication to combine
/// the trusted setup with the blob polynomial.
fn blob_to_kzg_commitment(blob: Blob, kzg_settings: &KzgSettings) -> Result<KzgCommitment, Error> {
    let bytes = blob.0.as_ref();
    let blob = c_kzg::Blob::from_bytes(bytes).unwrap();
    let commit = c_kzg::KzgCommitment::blob_to_kzg_commitment(&blob, kzg_settings)?;

    let bytes_commit = commit.to_bytes();
    let bytevector_commit = ByteVector::try_from(bytes_commit.as_ref()).unwrap();

    Ok(KzgCommitment(bytevector_commit))
}

/// Compute KZG proof at point 'z' for the polynomial represented by 'blob'.
/// Do this by computing the qoutient polynomial in evaluation form: q(x) = (p(x) - p(z)) / (x - z).
fn compute_kzg_proof(blob: &Blob, context: u8) -> Result<KzgProof, Error> {
    // TODO: Create context struct

    // KzgProof::compute_kzg_proof(blob, context.trusted_setup).map_err(Into::into)
    unimplemented!()
}
fn compute_blob_kzg_proof() {}
fn verify_kzg_proof() {}
fn verify_blob_kzg_proof() {}
fn verify_blob_kzg_proof_batch() {}
