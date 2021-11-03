use crate::primitives::{Domain, Root};
use ssz_rs::prelude::*;

pub enum DomainType {
    BeaconProposer,
    BeaconAttester,
    RANDAO,
    Deposit,
    VoluntaryExit,
    SelectionProof,
    AggregateAndProof,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SigningData {
    pub object_root: Root,
    pub domain: Domain,
}
