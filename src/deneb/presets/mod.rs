pub mod mainnet;
pub mod minimal;

pub struct Preset {
    pub max_blobs_per_block: usize,
    pub field_elements_per_blob: usize,
}
