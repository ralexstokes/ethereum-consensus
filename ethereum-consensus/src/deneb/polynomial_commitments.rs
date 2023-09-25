#![allow(unused)]
use crate::{crypto::hash, primitives, ssz::prelude::ByteVector};
use alloy_primitives::{uint, U256};
use blst::min_pk::PublicKey;
use c_kzg::{Bytes32, Bytes48, Error, KzgSettings};
use ssz_rs::prelude::*;
use std::ops::Deref;

pub const BLS_MODULUS: U256 =
    uint!(52435875175126190479447740508185965837690552500527637822603658699938581184513_U256);
pub const BYTES_PER_BLOB: usize = 32 * 4096;
pub const BYTES_PER_CONTEXT: usize = 10;
pub const BYTES_PER_COMMITMENT: usize = 48;
pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BYTES_PER_PROOF: usize = 48;
pub const KZG_COMMITMENT_BYTES_LEN: usize = 48;
pub const KZG_PROOF_BYTES_LEN: usize = 48;

pub type VersionedHash = primitives::Bytes32;
pub type BLSFieldElement = U256;
pub type Polynomial = Vec<BLSFieldElement>; // Should this polynomial type be an array?

const fn create_g1_point_at_infinity() -> [u8; 48] {
    let mut arr: [u8; 48] = [0; 48];
    arr[0] = 0xc0;
    arr
}

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

fn blob_to_kzg_commitment(blob: Blob, kzg_settings: &KzgSettings) -> Result<KzgCommitment, Error> {
    let inner = &blob.0;
    let blob = c_kzg::Blob::from_bytes(inner.as_ref()).unwrap();

    let commitment = c_kzg::KzgCommitment::blob_to_kzg_commitment(&blob, kzg_settings)?;
    let inner = ByteVector::try_from(commitment.to_bytes().as_slice()).unwrap();

    Ok(KzgCommitment(inner))
}

fn compute_kzg_proof(
    blob: Blob,
    z_bytes: Bytes32,
    kzg_settings: &KzgSettings,
) -> Result<(KzgProof, Bytes32), Error> {
    let inner = blob.0.as_ref();
    let blob = c_kzg::Blob::from_bytes(inner).unwrap();

    let (proof, evaluation) = c_kzg::KzgProof::compute_kzg_proof(&blob, &z_bytes, kzg_settings)?;
    let proof = ByteVector::try_from(proof.to_bytes().as_ref()).unwrap();

    Ok((KzgProof(proof), evaluation))
}

fn compute_blob_kzg_proof(
    blob: Blob,
    commitment_bytes: Bytes48,
    kzg_settings: &KzgSettings,
) -> Result<KzgProof, Error> {
    let inner = blob.0.as_ref();
    let blob = c_kzg::Blob::from_bytes(inner).unwrap();

    let ckzg_proof =
        c_kzg::KzgProof::compute_blob_kzg_proof(&blob, &commitment_bytes, kzg_settings)?;

    let bytes_proof = ckzg_proof.to_bytes();
    let proof = ByteVector::try_from(bytes_proof.as_ref()).unwrap();

    Ok(KzgProof(proof))
}

fn verify_kzg_proof(
    commitment_bytes: Bytes48,
    z_bytes: Bytes32,
    y_bytes: Bytes32,
    proof_bytes: Bytes48,
    kzg_settings: &KzgSettings,
) -> Result<bool, Error> {
    let out = c_kzg::KzgProof::verify_kzg_proof(
        &commitment_bytes,
        &z_bytes,
        &y_bytes,
        &proof_bytes,
        &kzg_settings,
    )?;

    Ok(out)
}

fn verify_blob_kzg_proof(
    blob: Blob,
    commitment_bytes: Bytes48,
    proof_bytes: Bytes48,
    kzg_settings: &KzgSettings,
) -> Result<bool, Error> {
    let bytes = blob.0.as_ref();
    let blob = c_kzg::Blob::from_bytes(bytes).unwrap();

    let out = c_kzg::KzgProof::verify_blob_kzg_proof(
        &blob,
        &commitment_bytes,
        &proof_bytes,
        &kzg_settings,
    )?;

    Ok(out)
}

fn verify_blob_kzg_proof_batch(
    blobs: &[Blob],
    commitments_bytes: &[Bytes48],
    proofs_bytes: &[Bytes48],
    kzg_settings: &KzgSettings,
) -> Result<bool, Error> {
    let mut c_kzg_blobs = Vec::with_capacity(blobs.len());

    for bytes in blobs.iter().map(|blob| blob.0.as_ref()) {
        let blob = c_kzg::Blob::from_bytes(bytes)?;
        c_kzg_blobs.push(blob);
    }

    let out = c_kzg::KzgProof::verify_blob_kzg_proof_batch(
        &c_kzg_blobs,
        commitments_bytes,
        proofs_bytes,
        kzg_settings,
    )?;

    Ok(out)
}
