use crate::{
    deneb::{
        polynomial_commitments::{KzgCommitment, KzgProof},
        SignedBeaconBlockHeader,
    },
    primitives::{BlobIndex, Bytes32, Root},
    ssz::prelude::*,
    Error,
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

impl<const BYTES_PER_BLOB: usize, const KZG_COMMITMENT_INCLUSION_PROOF_DEPTH: usize>
    BlobSidecar<BYTES_PER_BLOB, KZG_COMMITMENT_INCLUSION_PROOF_DEPTH>
{
    pub fn verify_blob_sidecar_inclusion_proof(&mut self) -> Result<(), Error> {
        // TODO: Calculate real gindex
        let index = 4;
        let leaf = self.kzg_commitment.hash_tree_root()?;
        let branch = self.kzg_commitment_inclusion_proof.as_ref();
        let depth = KZG_COMMITMENT_INCLUSION_PROOF_DEPTH;
        let root = self.signed_block_header.message.body_root;

        Ok(is_valid_merkle_branch(leaf, branch, depth, index, root)?)
    }
}

fn generalized_index_for_blob_index(index: usize) -> usize {
    57
}

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct BlobIdentifier {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_str")]
    pub index: BlobIndex,
}
