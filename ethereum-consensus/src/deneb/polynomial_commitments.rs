use crate::{deneb::blob_sidecar::Blob, primitives::Bytes32, ssz::prelude::*};
pub use c_kzg::KzgSettings;
use c_kzg::{Bytes48, Error};

use super::mainnet::FIELD_ELEMENTS_PER_BLOB;

pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BYTES_PER_BLOB: usize = BYTES_PER_FIELD_ELEMENT * FIELD_ELEMENTS_PER_BLOB;
pub const BYTES_PER_CONTEXT: usize = 10;
pub const BYTES_PER_COMMITMENT: usize = 48;
pub const BYTES_PER_PROOF: usize = 48;
pub const KZG_COMMITMENT_BYTES_LEN: usize = 48;
pub const KZG_PROOF_BYTES_LEN: usize = 48;

pub type VersionedHash = Bytes32;
pub type BLSFieldElement = U256;
pub type Polynomial = Vec<BLSFieldElement>;
pub type FieldElement = Bytes32;

pub struct ProofAndEvaluation {
    pub proof: KzgProof,
    pub evaluation: Bytes32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KzgCommitment(pub(crate) ByteVector<BYTES_PER_COMMITMENT>);

#[derive(Default, Debug, Clone, PartialEq, Eq, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KzgProof(ByteVector<BYTES_PER_PROOF>);

// TODO: Map error
pub fn blob_to_kzg_commitment<const BYTES_PER_BLOB: usize>(
    blob: &Blob<BYTES_PER_BLOB>,
    kzg_settings: &KzgSettings,
) -> Result<KzgCommitment, Error> {
    let blob = c_kzg::Blob::from_bytes(blob.as_ref()).unwrap();

    let commitment = c_kzg::KzgCommitment::blob_to_kzg_commitment(&blob, kzg_settings)?;
    let inner = ByteVector::try_from(commitment.to_bytes().as_slice()).unwrap();

    Ok(KzgCommitment(inner))
}

// TODO: Map error
pub fn compute_kzg_proof<const BYTES_PER_BLOB: usize>(
    blob: &Blob<BYTES_PER_BLOB>,
    evaluation_point: &FieldElement,
    kzg_settings: &KzgSettings,
) -> Result<ProofAndEvaluation, Error> {
    let blob = c_kzg::Blob::from_bytes(blob.as_ref()).unwrap();
    let evaluation_point = c_kzg::Bytes32::from_bytes(evaluation_point.as_ref())?;

    let (proof, evaluation) =
        c_kzg::KzgProof::compute_kzg_proof(&blob, &evaluation_point, kzg_settings)?;
    let proof = KzgProof(ByteVector::try_from(proof.to_bytes().as_ref()).unwrap());
    let evaluation = ByteVector::try_from(evaluation.as_slice()).unwrap();

    let result = ProofAndEvaluation { proof, evaluation };
    Ok(result)
}

// TODO: Map error
pub fn compute_blob_kzg_proof<const BYTES_PER_BLOB: usize>(
    blob: &Blob<BYTES_PER_BLOB>,
    commitment_bytes: &Bytes48,
    kzg_settings: &KzgSettings,
) -> Result<KzgProof, Error> {
    let blob = c_kzg::Blob::from_bytes(blob.as_ref()).unwrap();

    let ckzg_proof =
        c_kzg::KzgProof::compute_blob_kzg_proof(&blob, &commitment_bytes, kzg_settings)?;

    let bytes_proof = ckzg_proof.to_bytes();
    let proof = ByteVector::try_from(bytes_proof.as_ref()).unwrap();
    Ok(KzgProof(proof))
}

// TODO: Map error
pub fn verify_kzg_proof(
    commitment_bytes: &Bytes48,
    evaluation_point: &FieldElement,
    result_point: &FieldElement,
    proof_bytes: &Bytes48,
    kzg_settings: &KzgSettings,
) -> Result<bool, Error> {
    let evaluation_point = c_kzg::Bytes32::from_bytes(evaluation_point.as_ref())?;
    let result_point = c_kzg::Bytes32::from_bytes(result_point.as_ref())?;

    let out = c_kzg::KzgProof::verify_kzg_proof(
        commitment_bytes,
        &evaluation_point,
        &result_point,
        proof_bytes,
        kzg_settings,
    )?;
    Ok(out)
}

// TODO: Map error
pub fn verify_blob_kzg_proof<const BYTES_PER_BLOB: usize>(
    blob: &Blob<BYTES_PER_BLOB>,
    commitment_bytes: &Bytes48,
    proof_bytes: &Bytes48,
    kzg_settings: &KzgSettings,
) -> Result<bool, Error> {
    let blob = c_kzg::Blob::from_bytes(blob.as_ref()).unwrap();

    let out = c_kzg::KzgProof::verify_blob_kzg_proof(
        &blob,
        &commitment_bytes,
        &proof_bytes,
        kzg_settings,
    )?;
    Ok(out)
}

pub fn verify_blob_kzg_proof_batch<const BYTES_PER_BLOB: usize>(
    blobs: &[Blob<BYTES_PER_BLOB>],
    commitments_bytes: &[Bytes48],
    proofs_bytes: &[Bytes48],
    kzg_settings: &KzgSettings,
) -> Result<bool, Error> {
    let mut c_kzg_blobs = Vec::with_capacity(blobs.len());

    for blob in blobs {
        let blob = c_kzg::Blob::from_bytes(blob.as_ref())?;
        c_kzg_blobs.push(blob);
    }

    c_kzg::KzgProof::verify_blob_kzg_proof_batch(
        &c_kzg_blobs,
        commitments_bytes,
        proofs_bytes,
        kzg_settings,
    )
    .map_err(Into::into)
}
