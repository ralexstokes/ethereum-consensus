use crate::{
    deneb::polynomial_commitments::{KzgCommitment, KzgProof},
    primitives::{BlobIndex, BlsSignature, Root, Slot, ValidatorIndex},
    ssz::prelude::*,
};

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct BlindedBlobSidecar {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_str")]
    pub index: BlobIndex,
    #[serde(with = "crate::serde::as_str")]
    pub slot: Slot,
    pub block_parent_root: Root,
    #[serde(with = "crate::serde::as_str")]
    pub proposer_index: ValidatorIndex,
    pub blob_root: Root,
    pub kzg_commitment: KzgCommitment,
    pub kzg_proof: KzgProof,
}

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct SignedBlindedBlobSidecar {
    pub message: BlindedBlobSidecar,
    pub signature: BlsSignature,
}
