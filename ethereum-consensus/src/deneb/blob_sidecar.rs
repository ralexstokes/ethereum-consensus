use crate::{
    kzg::{KzgCommitment, KzgProof},
    primitives::{BlobIndex, BlsSignature, Root, Slot, ValidatorIndex},
    ssz::prelude::*,
};

pub const BLOB_TX_TYPE: u8 = 3;
pub const VERSIONED_HASH_VERSION_KZG: u8 = 1;

pub type Blob<const BYTES_PER_BLOB: usize> = ByteVector<BYTES_PER_BLOB>;

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct BlobSidecar<const BYTES_PER_BLOB: usize> {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub index: BlobIndex,
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    pub block_parent_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub proposer_index: ValidatorIndex,
    pub blob: Blob<BYTES_PER_BLOB>,
    pub kzg_commitment: KzgCommitment,
    pub kzg_proof: KzgProof,
}

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct SignedBlobSidecar<const BYTES_PER_BLOB: usize> {
    pub message: BlobSidecar<BYTES_PER_BLOB>,
    pub signature: BlsSignature,
}

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct BlobIdentifier {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub index: BlobIndex,
}
