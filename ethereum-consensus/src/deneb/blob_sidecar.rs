use crate::{
    deneb::{
        beacon_block::BeaconBlockBody,
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
pub fn verify_blob_sidecar_inclusion_proof<
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
    const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
    const KZG_COMMITMENT_INCLUSION_PROOF_DEPTH: usize,
>(
    index: BlobIndex,
    kzg_commitment: &mut KzgCommitment,
    signed_block_header: &SignedBeaconBlockHeader,
    kzg_commitment_inclusion_proof: &Vector<Bytes32, KZG_COMMITMENT_INCLUSION_PROOF_DEPTH>,
) -> Result<(), Error> {
    let g_index = generalized_index_for_blob_index::<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
    >(index)?;
    let subtree_index = get_subtree_index(g_index)?;

    let leaf = kzg_commitment.hash_tree_root()?;
    let branch = kzg_commitment_inclusion_proof.as_ref();
    let depth = KZG_COMMITMENT_INCLUSION_PROOF_DEPTH;
    let root = signed_block_header.message.body_root;

    Ok(is_valid_merkle_branch(leaf, branch, depth, subtree_index as usize, root)?)
}

fn generalized_index_for_blob_index<
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
    const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
>(
    i: usize,
) -> Result<GeneralizedIndex, MerkleizationError> {
    let path = &["blob_kzg_commitments".into(), i.into()];
    BeaconBlockBody::<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
    >::generalized_index(path)
}

fn get_subtree_index(i: GeneralizedIndex) -> Result<u32, MerkleizationError> {
    match i.checked_ilog2() {
        Some(floorlog2) => Ok(floorlog2),
        None => Err(MerkleizationError::InvalidGeneralizedIndex),
    }
}

#[derive(
    Default, Debug, Clone, SimpleSerialize, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct BlobIdentifier {
    pub block_root: Root,
    #[serde(with = "crate::serde::as_str")]
    pub index: BlobIndex,
}
