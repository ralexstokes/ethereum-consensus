use crate::primitives::{Domain, Root};
use ssz_rs::prelude::*;

#[derive(Clone, Copy)]
pub enum DomainType {
    BeaconProposer,
    BeaconAttester,
    Randao,
    Deposit,
    VoluntaryExit,
    SelectionProof,
    AggregateAndProof,
    ApplicationMask,
}

impl DomainType {
    pub fn as_bytes(&self) -> [u8; 4] {
        match self {
            Self::ApplicationMask => [0, 0, 0, 1u8],
            _ => {
                let data = *self as u32;
                data.to_le_bytes()
            }
        }
    }
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SigningData {
    pub object_root: Root,
    pub domain: Domain,
}
