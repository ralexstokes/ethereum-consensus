use crate::primitives::{Domain, Root};
use ssz_rs::prelude::*;

pub enum DomainType {
    BeaconProposer,
    BeaconAttester,
    Randao,
    Deposit,
    VoluntaryExit,
    SelectionProof,
    AggregateAndProof,
}

impl DomainType {
    pub fn get_domain_constant(&self) -> [u8; 4] {
        match self {
            DomainType::BeaconProposer => 0u32.to_le_bytes(),
            DomainType::BeaconAttester => 1u32.to_le_bytes(),
            DomainType::Randao => 2u32.to_le_bytes(),
            DomainType::Deposit => 3u32.to_le_bytes(),
            DomainType::VoluntaryExit => 4u32.to_le_bytes(),
            DomainType::SelectionProof => 5u32.to_le_bytes(),
            DomainType::AggregateAndProof => 6u32.to_le_bytes(),
        }
    }
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SigningData {
    pub object_root: Root,
    pub domain: Domain,
}
