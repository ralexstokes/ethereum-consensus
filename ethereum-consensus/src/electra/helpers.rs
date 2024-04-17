use ssz_rs::Bitvector;

use crate::capella::CommitteeIndex;

pub fn get_committee_indices<const MAX_COMMITTEES_PER_SLOT: usize>(
    committee_bits: Bitvector<MAX_COMMITTEES_PER_SLOT>,
) -> Vec<CommitteeIndex> {
    committee_bits.iter().enumerate().flat_map(|(i, bit)| bit.then_some(i)).collect()
}
