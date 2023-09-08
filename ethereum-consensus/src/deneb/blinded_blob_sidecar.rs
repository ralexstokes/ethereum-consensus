use crate::{
    kzg::{KzgCommitment, KzgProof},
    primitives::{BlobIndex, BlsSignature, Root, Slot, ValidatorIndex},
};
use ssz_rs::prelude::*;

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlindedBlobSidecar<const BYTES_PER_BLOB: usize> {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub index: BlobIndex,
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    pub block_parent_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub proposer_index: ValidatorIndex,
    pub blob_root: Root,
    pub kzg_commitment: KzgCommitment,
    pub kzg_proof: KzgProof,
}

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedBlindedBlobSidecar<const MAX_BLOBS_PER_BLOCK: usize, const BYTES_PER_BLOB: usize> {
    pub message: BlindedBlobSidecar<BYTES_PER_BLOB>,
    pub signature: BlsSignature,
}
