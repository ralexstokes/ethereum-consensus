use crate::{
    deneb::{
        polynomial_commitments::{KzgCommitment, KzgProof},
        SignedBeaconBlockHeader,
    },
    primitives::{BlobIndex, Bytes32, Root},
    ssz::prelude::*,
};

pub const VERSIONED_HASH_VERSION_KZG: u8 = 1;

pub type Blob<const BYTES_PER_BLOB: usize> = ByteVector<BYTES_PER_BLOB>;

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct BlobSidecar<
    const BYTES_PER_BLOB: usize,
    const KZG_COMMITMENT_INCLUSION_PROOF_DEPTH: usize,
> {
    #[serde(with = "crate::serde::as_str")]
    pub index: BlobIndex,
    pub blob: Blob<BYTES_PER_BLOB>,
    pub kzg_commitment: KzgCommitment,
    pub kzg_proof: KzgProof,
    pub signed_block_header: SignedBeaconBlockHeader,
    pub kzg_commitment_inclusion_proof: Vector<Bytes32, KZG_COMMITMENT_INCLUSION_PROOF_DEPTH>,
}

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct BlobIdentifier {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_str")]
    pub index: BlobIndex,
}
