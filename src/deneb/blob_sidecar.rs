use crate::kzg::{KzgCommitment, KzgProof};
use crate::primitives::{BlobIndex, BlsSignature, Root, Slot, ValidatorIndex};
use crate::ssz::ByteVector;
use ssz_rs::prelude::*;

pub type Blob<const BYTES_PER_BLOB: usize> = ByteVector<BYTES_PER_BLOB>;

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlobSidecar<const BYTES_PER_BLOB: usize> {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub index: BlobIndex,
    pub slot: Slot,
    pub block_parent_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub proposer_index: ValidatorIndex,
    pub blob: Blob<BYTES_PER_BLOB>,
    pub kzg_commitment: KzgCommitment,
    pub kzg_proof: KzgProof,
}

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedBlobSidecar<const MAX_BLOBS_PER_BLOCK: usize, const BYTES_PER_BLOB: usize> {
    pub message: BlobSidecar<BYTES_PER_BLOB>,
    pub signature: BlsSignature,
}

#[derive(Default, Debug, Clone, SimpleSerialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlobIdentifier {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub index: BlobIndex,
}
