//! WARNING: This file was derived by the `spec-gen` utility. DO NOT EDIT MANUALLY.
use crate::{
    bellatrix::blinded_beacon_block as bellatrix,
    capella::blinded_beacon_block as capella,
    deneb::blinded_beacon_block as deneb,
    electra::blinded_beacon_block as electra,
    primitives::{Root, Slot, ValidatorIndex},
    ssz::prelude::*,
    types::blinded_beacon_block_body::{BlindedBeaconBlockBodyRef, BlindedBeaconBlockBodyRefMut},
    Fork as Version,
};
#[derive(Debug, Clone, PartialEq, Eq, Serializable, HashTreeRoot, serde::Serialize)]
#[ssz(transparent)]
#[serde(untagged)]
pub enum BlindedBeaconBlock<
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
    const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
    const MAX_VALIDATORS_PER_SLOT: usize,
    const MAX_COMMITTEES_PER_SLOT: usize,
    const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
    const MAX_ATTESTATIONS_ELECTRA: usize,
    const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
    const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
> {
    Bellatrix(
        bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ),
    Capella(
        capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    ),
    Deneb(
        deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    ),
    Electra(
        electra::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_SLOT,
            MAX_COMMITTEES_PER_SLOT,
            MAX_ATTESTER_SLASHINGS_ELECTRA,
            MAX_ATTESTATIONS_ELECTRA,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    ),
}
impl<
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    >
    BlindedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn bellatrix_mut(
        &mut self,
    ) -> Option<
        &mut bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella(
        &self,
    ) -> Option<
        &capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella_mut(
        &mut self,
    ) -> Option<
        &mut capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb(
        &self,
    ) -> Option<
        &deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb_mut(
        &mut self,
    ) -> Option<
        &mut deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
            Self::Electra(_) => Version::Electra,
        }
    }
    pub fn slot(&self) -> Slot {
        match self {
            Self::Bellatrix(inner) => inner.slot,
            Self::Capella(inner) => inner.slot,
            Self::Deneb(inner) => inner.slot,
            Self::Electra(inner) => inner.slot,
        }
    }
    pub fn slot_mut(&mut self) -> &mut Slot {
        match self {
            Self::Bellatrix(inner) => &mut inner.slot,
            Self::Capella(inner) => &mut inner.slot,
            Self::Deneb(inner) => &mut inner.slot,
            Self::Electra(inner) => &mut inner.slot,
        }
    }
    pub fn proposer_index(&self) -> ValidatorIndex {
        match self {
            Self::Bellatrix(inner) => inner.proposer_index,
            Self::Capella(inner) => inner.proposer_index,
            Self::Deneb(inner) => inner.proposer_index,
            Self::Electra(inner) => inner.proposer_index,
        }
    }
    pub fn proposer_index_mut(&mut self) -> &mut ValidatorIndex {
        match self {
            Self::Bellatrix(inner) => &mut inner.proposer_index,
            Self::Capella(inner) => &mut inner.proposer_index,
            Self::Deneb(inner) => &mut inner.proposer_index,
            Self::Electra(inner) => &mut inner.proposer_index,
        }
    }
    pub fn parent_root(&self) -> Root {
        match self {
            Self::Bellatrix(inner) => inner.parent_root,
            Self::Capella(inner) => inner.parent_root,
            Self::Deneb(inner) => inner.parent_root,
            Self::Electra(inner) => inner.parent_root,
        }
    }
    pub fn parent_root_mut(&mut self) -> &mut Root {
        match self {
            Self::Bellatrix(inner) => &mut inner.parent_root,
            Self::Capella(inner) => &mut inner.parent_root,
            Self::Deneb(inner) => &mut inner.parent_root,
            Self::Electra(inner) => &mut inner.parent_root,
        }
    }
    pub fn state_root(&self) -> Root {
        match self {
            Self::Bellatrix(inner) => inner.state_root,
            Self::Capella(inner) => inner.state_root,
            Self::Deneb(inner) => inner.state_root,
            Self::Electra(inner) => inner.state_root,
        }
    }
    pub fn state_root_mut(&mut self) -> &mut Root {
        match self {
            Self::Bellatrix(inner) => &mut inner.state_root,
            Self::Capella(inner) => &mut inner.state_root,
            Self::Deneb(inner) => &mut inner.state_root,
            Self::Electra(inner) => &mut inner.state_root,
        }
    }
    pub fn body(
        &self,
    ) -> BlindedBeaconBlockBodyRef<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    > {
        match self {
            Self::Bellatrix(inner) => From::from(&inner.body),
            Self::Capella(inner) => From::from(&inner.body),
            Self::Deneb(inner) => From::from(&inner.body),
            Self::Electra(inner) => From::from(&inner.body),
        }
    }
    pub fn body_mut(
        &mut self,
    ) -> BlindedBeaconBlockBodyRefMut<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
    > {
        match self {
            Self::Bellatrix(inner) => From::from(&mut inner.body),
            Self::Capella(inner) => From::from(&mut inner.body),
            Self::Deneb(inner) => From::from(&mut inner.body),
            Self::Electra(_) => unimplemented!("fork not supported yet"),
        }
    }
}
impl<
        'de,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    > serde::Deserialize<'de>
    for BlindedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    >
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
            return Ok(Self::Electra(inner));
        }
        if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
            return Ok(Self::Deneb(inner));
        }
        if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
            return Ok(Self::Capella(inner));
        }
        if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
            return Ok(Self::Bellatrix(inner));
        }
        Err(serde::de::Error::custom("no variant could be deserialized from input"))
    }
}
#[derive(Debug, PartialEq, Eq, HashTreeRoot)]
#[ssz(transparent)]
pub enum BlindedBeaconBlockRef<
    'a,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
    const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
    const MAX_VALIDATORS_PER_SLOT: usize,
    const MAX_COMMITTEES_PER_SLOT: usize,
    const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
    const MAX_ATTESTATIONS_ELECTRA: usize,
    const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
    const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
> {
    Bellatrix(
        &'a bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ),
    Capella(
        &'a capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    ),
    Deneb(
        &'a deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    ),
    Electra(
        &'a electra::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_SLOT,
            MAX_COMMITTEES_PER_SLOT,
            MAX_ATTESTER_SLASHINGS_ELECTRA,
            MAX_ATTESTATIONS_ELECTRA,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    ),
}
impl<
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    >
    BlindedBeaconBlockRef<
        '_,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella(
        &self,
    ) -> Option<
        &capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb(
        &self,
    ) -> Option<
        &deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
            Self::Electra(_) => Version::Electra,
        }
    }
    pub fn slot(&self) -> Slot {
        match self {
            Self::Bellatrix(inner) => inner.slot,
            Self::Capella(inner) => inner.slot,
            Self::Deneb(inner) => inner.slot,
            Self::Electra(inner) => inner.slot,
        }
    }
    pub fn proposer_index(&self) -> ValidatorIndex {
        match self {
            Self::Bellatrix(inner) => inner.proposer_index,
            Self::Capella(inner) => inner.proposer_index,
            Self::Deneb(inner) => inner.proposer_index,
            Self::Electra(inner) => inner.proposer_index,
        }
    }
    pub fn parent_root(&self) -> Root {
        match self {
            Self::Bellatrix(inner) => inner.parent_root,
            Self::Capella(inner) => inner.parent_root,
            Self::Deneb(inner) => inner.parent_root,
            Self::Electra(inner) => inner.parent_root,
        }
    }
    pub fn state_root(&self) -> Root {
        match self {
            Self::Bellatrix(inner) => inner.state_root,
            Self::Capella(inner) => inner.state_root,
            Self::Deneb(inner) => inner.state_root,
            Self::Electra(inner) => inner.state_root,
        }
    }
    pub fn body(
        &self,
    ) -> BlindedBeaconBlockBodyRef<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    > {
        match self {
            Self::Bellatrix(inner) => From::from(&inner.body),
            Self::Capella(inner) => From::from(&inner.body),
            Self::Deneb(inner) => From::from(&inner.body),
            Self::Electra(inner) => From::from(&inner.body),
        }
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    >
    From<
        &'a bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    >
    for BlindedBeaconBlockRef<
        'a,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ) -> Self {
        Self::Bellatrix(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    >
    From<
        &'a capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    >
    for BlindedBeaconBlockRef<
        'a,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    ) -> Self {
        Self::Capella(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    >
    From<
        &'a deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    >
    for BlindedBeaconBlockRef<
        'a,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    ) -> Self {
        Self::Deneb(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_VALIDATORS_PER_SLOT: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    >
    From<
        &'a electra::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_SLOT,
            MAX_COMMITTEES_PER_SLOT,
            MAX_ATTESTER_SLASHINGS_ELECTRA,
            MAX_ATTESTATIONS_ELECTRA,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    >
    for BlindedBeaconBlockRef<
        'a,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    >
{
    fn from(
        value: &'a electra::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_SLOT,
            MAX_COMMITTEES_PER_SLOT,
            MAX_ATTESTER_SLASHINGS_ELECTRA,
            MAX_ATTESTATIONS_ELECTRA,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
            MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
            MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
            MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
        >,
    ) -> Self {
        Self::Electra(value)
    }
}
#[derive(Debug, PartialEq, Eq, HashTreeRoot)]
#[ssz(transparent)]
pub enum BlindedBeaconBlockRefMut<
    'a,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
    const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
    const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
    const MAX_ATTESTATIONS_ELECTRA: usize,
> {
    Bellatrix(
        &'a mut bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ),
    Capella(
        &'a mut capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    ),
    Deneb(
        &'a mut deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    ),
}
impl<
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
    >
    BlindedBeaconBlockRefMut<
        '_,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn bellatrix_mut(
        &mut self,
    ) -> Option<
        &mut bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    > {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella(
        &self,
    ) -> Option<
        &capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn capella_mut(
        &mut self,
    ) -> Option<
        &mut capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    > {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb(
        &self,
    ) -> Option<
        &deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn deneb_mut(
        &mut self,
    ) -> Option<
        &mut deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    > {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            Self::Bellatrix(_) => Version::Bellatrix,
            Self::Capella(_) => Version::Capella,
            Self::Deneb(_) => Version::Deneb,
        }
    }
    pub fn slot(&self) -> Slot {
        match self {
            Self::Bellatrix(inner) => inner.slot,
            Self::Capella(inner) => inner.slot,
            Self::Deneb(inner) => inner.slot,
        }
    }
    pub fn slot_mut(&mut self) -> &mut Slot {
        match self {
            Self::Bellatrix(inner) => &mut inner.slot,
            Self::Capella(inner) => &mut inner.slot,
            Self::Deneb(inner) => &mut inner.slot,
        }
    }
    pub fn proposer_index(&self) -> ValidatorIndex {
        match self {
            Self::Bellatrix(inner) => inner.proposer_index,
            Self::Capella(inner) => inner.proposer_index,
            Self::Deneb(inner) => inner.proposer_index,
        }
    }
    pub fn proposer_index_mut(&mut self) -> &mut ValidatorIndex {
        match self {
            Self::Bellatrix(inner) => &mut inner.proposer_index,
            Self::Capella(inner) => &mut inner.proposer_index,
            Self::Deneb(inner) => &mut inner.proposer_index,
        }
    }
    pub fn parent_root(&self) -> Root {
        match self {
            Self::Bellatrix(inner) => inner.parent_root,
            Self::Capella(inner) => inner.parent_root,
            Self::Deneb(inner) => inner.parent_root,
        }
    }
    pub fn parent_root_mut(&mut self) -> &mut Root {
        match self {
            Self::Bellatrix(inner) => &mut inner.parent_root,
            Self::Capella(inner) => &mut inner.parent_root,
            Self::Deneb(inner) => &mut inner.parent_root,
        }
    }
    pub fn state_root(&self) -> Root {
        match self {
            Self::Bellatrix(inner) => inner.state_root,
            Self::Capella(inner) => inner.state_root,
            Self::Deneb(inner) => inner.state_root,
        }
    }
    pub fn state_root_mut(&mut self) -> &mut Root {
        match self {
            Self::Bellatrix(inner) => &mut inner.state_root,
            Self::Capella(inner) => &mut inner.state_root,
            Self::Deneb(inner) => &mut inner.state_root,
        }
    }
    pub fn body<
        const MAX_VALIDATORS_PER_SLOT: usize,
        const MAX_COMMITTEES_PER_SLOT: usize,
        const MAX_DEPOSIT_REQUESTS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD: usize,
        const MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD: usize,
    >(
        &self,
    ) -> BlindedBeaconBlockBodyRef<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_VALIDATORS_PER_SLOT,
        MAX_COMMITTEES_PER_SLOT,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
        MAX_DEPOSIT_REQUESTS_PER_PAYLOAD,
        MAX_WITHDRAWAL_REQUESTS_PER_PAYLOAD,
        MAX_CONSOLIDATION_REQUESTS_PER_PAYLOAD,
    > {
        match self {
            Self::Bellatrix(inner) => From::from(&inner.body),
            Self::Capella(inner) => From::from(&inner.body),
            Self::Deneb(inner) => From::from(&inner.body),
        }
    }
    pub fn body_mut(
        &mut self,
    ) -> BlindedBeaconBlockBodyRefMut<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
    > {
        match self {
            Self::Bellatrix(inner) => From::from(&mut inner.body),
            Self::Capella(inner) => From::from(&mut inner.body),
            Self::Deneb(inner) => From::from(&mut inner.body),
        }
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
    >
    From<
        &'a mut bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    >
    for BlindedBeaconBlockRefMut<
        'a,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
    >
{
    fn from(
        value: &'a mut bellatrix::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >,
    ) -> Self {
        Self::Bellatrix(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
    >
    From<
        &'a mut capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    >
    for BlindedBeaconBlockRefMut<
        'a,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
    >
{
    fn from(
        value: &'a mut capella::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
        >,
    ) -> Self {
        Self::Capella(value)
    }
}
impl<
        'a,
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
        const SYNC_COMMITTEE_SIZE: usize,
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BLS_TO_EXECUTION_CHANGES: usize,
        const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize,
        const MAX_ATTESTER_SLASHINGS_ELECTRA: usize,
        const MAX_ATTESTATIONS_ELECTRA: usize,
    >
    From<
        &'a mut deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    >
    for BlindedBeaconBlockRefMut<
        'a,
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BLS_TO_EXECUTION_CHANGES,
        MAX_BLOB_COMMITMENTS_PER_BLOCK,
        MAX_ATTESTER_SLASHINGS_ELECTRA,
        MAX_ATTESTATIONS_ELECTRA,
    >
{
    fn from(
        value: &'a mut deneb::BlindedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BLS_TO_EXECUTION_CHANGES,
            MAX_BLOB_COMMITMENTS_PER_BLOCK,
        >,
    ) -> Self {
        Self::Deneb(value)
    }
}
