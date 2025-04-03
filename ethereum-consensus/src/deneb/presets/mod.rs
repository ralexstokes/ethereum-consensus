pub mod mainnet;
pub mod minimal;

pub struct Preset {
    pub field_elements_per_blob: usize,
    pub max_blob_commitments_per_block: usize,
    pub max_blobs_per_block: usize,
}

/// Precompute value that optimizes computing cell kzg proofs.
///
/// Set to 0 as we do not use `compute_cells_and_kzg_proofs` or `recover_cells_and_kzg_proofs`.
///
/// Learn more: <https://github.com/ethereum/c-kzg-4844/blob/dffa18ee350aeef38f749ffad24a27c1645fb4f8/README.md?plain=1#L112>
pub const PRECOMPUTE: u64 = 0;
