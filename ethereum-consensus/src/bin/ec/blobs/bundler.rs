use crate::blobs::{Blob, Error};
use ethereum_consensus::{
    deneb::{self, polynomial_commitments as spec, presets::TRUSTED_SETUP_JSON},
    Error as ConsensusError,
};
use std::io::Read;

type BlobsBundle = deneb::mainnet::BlobsBundle;
type Commitment = spec::KzgCommitment;
type Proof = spec::KzgProof;
type CommitmentAndProof = (Commitment, Proof);

pub fn commit_and_prove_blob(
    blob: &Blob,
    kzg_settings: &spec::KzgSettings,
) -> Result<CommitmentAndProof, ConsensusError> {
    let commitment = spec::blob_to_kzg_commitment(blob, kzg_settings)?;
    let proof = spec::compute_blob_kzg_proof(blob, &commitment, kzg_settings)?;
    Ok((commitment, proof))
}

pub fn bundle(blobs: Vec<Blob>, kzg_settings: &spec::KzgSettings) -> Result<BlobsBundle, Error> {
    let commitments_and_proofs = blobs
        .iter()
        .map(|blob| commit_and_prove_blob(blob, kzg_settings))
        .collect::<Result<Vec<CommitmentAndProof>, ConsensusError>>()?;
    let (commitments, proofs) = commitments_and_proofs.into_iter().unzip();
    let blobs_bundle = BlobsBundle { commitments, proofs, blobs };

    spec::verify_blob_kzg_proof_batch(
        &blobs_bundle.blobs,
        &blobs_bundle.commitments,
        &blobs_bundle.proofs,
        kzg_settings,
    )
    .map_err(ConsensusError::from)?;

    Ok(blobs_bundle)
}

// Assumes a serde_json-encoded array of `Vec<Blob>` on `reader` and uses the mainnet trusted setup.
pub fn from_reader(reader: impl Read) -> Result<BlobsBundle, Error> {
    let kzg_settings = spec::kzg_settings_from_json(TRUSTED_SETUP_JSON)?;
    let blobs: Vec<Blob> = serde_json::from_reader(reader)?;
    bundle(blobs, &kzg_settings)
}
