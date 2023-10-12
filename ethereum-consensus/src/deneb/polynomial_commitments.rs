use crate::{deneb::blob_sidecar::Blob, primitives::Bytes32, ssz::prelude::*};
pub use c_kzg::KzgSettings;
use thiserror::Error;

pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BYTES_PER_COMMITMENT: usize = 48;
pub const BYTES_PER_PROOF: usize = 48;

pub type VersionedHash = Bytes32;
pub type FieldElement = Bytes32;
pub type KzgCommitment = ByteVector<BYTES_PER_COMMITMENT>;
pub type KzgProof = ByteVector<BYTES_PER_PROOF>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CKzg(#[from] c_kzg::Error),
    #[error("proof verification failed")]
    InvalidProof,
}

pub struct ProofAndEvaluation {
    pub proof: KzgProof,
    pub evaluation: FieldElement,
}

pub fn blob_to_kzg_commitment<const BYTES_PER_BLOB: usize>(
    blob: &Blob<BYTES_PER_BLOB>,
    kzg_settings: &KzgSettings,
) -> Result<KzgCommitment, Error> {
    let blob = c_kzg::Blob::from_bytes(blob.as_ref())?;

    let commitment = c_kzg::KzgCommitment::blob_to_kzg_commitment(&blob, kzg_settings)?;
    let inner = KzgCommitment::try_from(commitment.to_bytes().as_slice()).expect("correct size");
    Ok(inner)
}

pub fn compute_kzg_proof<const BYTES_PER_BLOB: usize>(
    blob: &Blob<BYTES_PER_BLOB>,
    evaluation_point: &FieldElement,
    kzg_settings: &KzgSettings,
) -> Result<ProofAndEvaluation, Error> {
    let blob = c_kzg::Blob::from_bytes(blob.as_ref())?;
    let evaluation_point = c_kzg::Bytes32::from_bytes(evaluation_point.as_ref())?;

    let (proof, evaluation) =
        c_kzg::KzgProof::compute_kzg_proof(&blob, &evaluation_point, kzg_settings)?;
    let proof = KzgProof::try_from(proof.to_bytes().as_ref()).expect("correct size");
    let evaluation = FieldElement::try_from(evaluation.as_slice()).expect("correct size");

    let result = ProofAndEvaluation { proof, evaluation };
    Ok(result)
}

pub fn compute_blob_kzg_proof<const BYTES_PER_BLOB: usize>(
    blob: &Blob<BYTES_PER_BLOB>,
    commitment: &KzgCommitment,
    kzg_settings: &KzgSettings,
) -> Result<KzgProof, Error> {
    let blob = c_kzg::Blob::from_bytes(blob.as_ref())?;
    let commitment = c_kzg::Bytes48::from_bytes(commitment.as_ref()).expect("correct size");

    let proof = c_kzg::KzgProof::compute_blob_kzg_proof(&blob, &commitment, kzg_settings)?;

    Ok(KzgProof::try_from(proof.to_bytes().as_ref()).expect("input is correct size"))
}

pub fn verify_kzg_proof(
    commitment: &KzgCommitment,
    evaluation_point: &FieldElement,
    result_point: &FieldElement,
    proof: &KzgProof,
    kzg_settings: &KzgSettings,
) -> Result<(), Error> {
    let evaluation_point = c_kzg::Bytes32::from_bytes(evaluation_point.as_ref())?;
    let result_point = c_kzg::Bytes32::from_bytes(result_point.as_ref())?;
    let commitment = c_kzg::Bytes48::from_bytes(commitment.as_ref()).expect("correct size");
    let proof = c_kzg::Bytes48::from_bytes(proof.as_ref()).expect("correct size");

    let res = c_kzg::KzgProof::verify_kzg_proof(
        &commitment,
        &evaluation_point,
        &result_point,
        &proof,
        kzg_settings,
    )?;

    res.then_some(()).ok_or(Error::InvalidProof)
}

pub fn verify_blob_kzg_proof<const BYTES_PER_BLOB: usize>(
    blob: &Blob<BYTES_PER_BLOB>,
    commitment: &KzgCommitment,
    proof: &KzgProof,
    kzg_settings: &KzgSettings,
) -> Result<(), Error> {
    let blob = c_kzg::Blob::from_bytes(blob.as_ref())?;
    let commitment = c_kzg::Bytes48::from_bytes(commitment.as_ref()).unwrap();
    let proof = c_kzg::Bytes48::from_bytes(proof.as_ref()).unwrap();

    let res = c_kzg::KzgProof::verify_blob_kzg_proof(&blob, &commitment, &proof, kzg_settings)?;

    res.then_some(()).ok_or(Error::InvalidProof)
}

pub fn verify_blob_kzg_proof_batch<const BYTES_PER_BLOB: usize>(
    blobs: &[Blob<BYTES_PER_BLOB>],
    commitments: &[KzgCommitment],
    proofs: &[KzgProof],
    kzg_settings: &KzgSettings,
) -> Result<(), Error> {
    let mut c_kzg_blobs = Vec::with_capacity(blobs.len());
    let mut c_kzg_commitments = Vec::with_capacity(commitments.len());
    let mut c_kzg_proofs = Vec::with_capacity(proofs.len());

    for blob in blobs {
        let blob = c_kzg::Blob::from_bytes(blob.as_ref())?;
        c_kzg_blobs.push(blob);
    }
    for commitment in commitments {
        let commitment = c_kzg::Bytes48::from_bytes(commitment.as_ref()).unwrap();
        c_kzg_commitments.push(commitment);
    }
    for proof in proofs {
        let proof = c_kzg::Bytes48::from_bytes(proof.as_ref()).unwrap();
        c_kzg_proofs.push(proof);
    }

    let res = c_kzg::KzgProof::verify_blob_kzg_proof_batch(
        &c_kzg_blobs,
        &c_kzg_commitments,
        &c_kzg_proofs,
        kzg_settings,
    )?;

    res.then_some(()).ok_or(Error::InvalidProof)
}
