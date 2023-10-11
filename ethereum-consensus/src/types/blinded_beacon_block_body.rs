//! WARNING: This file was derived by the `spec-gen` utility. DO NOT EDIT MANUALLY.
use crate::{
    altair::SyncAggregate,
    bellatrix::blinded_beacon_block as bellatrix,
    capella::{blinded_beacon_block as capella, SignedBlsToExecutionChange},
    deneb::{blinded_beacon_block as deneb, polynomial_commitments::KzgCommitment},
    phase0::{
        Attestation, AttesterSlashing, Deposit, Eth1Data, ProposerSlashing, SignedVoluntaryExit,
    },
    primitives::{BlsSignature, Bytes32},
    ssz::prelude::*,
    types::execution_payload_header::{ExecutionPayloadHeaderRef, ExecutionPayloadHeaderRefMut},
    Fork as Version,
};
#[derive(Debug, Clone, PartialEq, Eq, SimpleSerialize, serde::Deserialize)]
#[serde(tag = "version", content = "data")]
#[serde(rename_all = "lowercase")]
pub enum BlindedBeaconBlockBody<
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
> {
    Bellatrix(
        bellatrix::BlindedBeaconBlockBody<
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
        capella::BlindedBeaconBlockBody<
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
        deneb::BlindedBeaconBlockBody<
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
    >
    BlindedBeaconBlockBody<
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
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::BlindedBeaconBlockBody<
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
        &mut bellatrix::BlindedBeaconBlockBody<
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
        &capella::BlindedBeaconBlockBody<
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
        &mut capella::BlindedBeaconBlockBody<
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
        &deneb::BlindedBeaconBlockBody<
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
        &mut deneb::BlindedBeaconBlockBody<
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
    pub fn randao_reveal(&self) -> Option<&BlsSignature> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.randao_reveal),
            Self::Capella(inner) => Some(&inner.randao_reveal),
            Self::Deneb(inner) => Some(&inner.randao_reveal),
        }
    }
    pub fn randao_reveal_mut(&mut self) -> Option<&mut BlsSignature> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.randao_reveal),
            Self::Capella(inner) => Some(&mut inner.randao_reveal),
            Self::Deneb(inner) => Some(&mut inner.randao_reveal),
        }
    }
    pub fn eth1_data(&self) -> Option<&Eth1Data> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.eth1_data),
            Self::Capella(inner) => Some(&inner.eth1_data),
            Self::Deneb(inner) => Some(&inner.eth1_data),
        }
    }
    pub fn eth1_data_mut(&mut self) -> Option<&mut Eth1Data> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.eth1_data),
            Self::Capella(inner) => Some(&mut inner.eth1_data),
            Self::Deneb(inner) => Some(&mut inner.eth1_data),
        }
    }
    pub fn graffiti(&self) -> Option<&Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.graffiti),
            Self::Capella(inner) => Some(&inner.graffiti),
            Self::Deneb(inner) => Some(&inner.graffiti),
        }
    }
    pub fn graffiti_mut(&mut self) -> Option<&mut Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.graffiti),
            Self::Capella(inner) => Some(&mut inner.graffiti),
            Self::Deneb(inner) => Some(&mut inner.graffiti),
        }
    }
    pub fn proposer_slashings(&self) -> Option<&List<ProposerSlashing, MAX_PROPOSER_SLASHINGS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.proposer_slashings),
            Self::Capella(inner) => Some(&inner.proposer_slashings),
            Self::Deneb(inner) => Some(&inner.proposer_slashings),
        }
    }
    pub fn proposer_slashings_mut(
        &mut self,
    ) -> Option<&mut List<ProposerSlashing, MAX_PROPOSER_SLASHINGS>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.proposer_slashings),
            Self::Capella(inner) => Some(&mut inner.proposer_slashings),
            Self::Deneb(inner) => Some(&mut inner.proposer_slashings),
        }
    }
    pub fn attester_slashings(
        &self,
    ) -> Option<&List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.attester_slashings),
            Self::Capella(inner) => Some(&inner.attester_slashings),
            Self::Deneb(inner) => Some(&inner.attester_slashings),
        }
    }
    pub fn attester_slashings_mut(
        &mut self,
    ) -> Option<&mut List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS>>
    {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.attester_slashings),
            Self::Capella(inner) => Some(&mut inner.attester_slashings),
            Self::Deneb(inner) => Some(&mut inner.attester_slashings),
        }
    }
    pub fn attestations(
        &self,
    ) -> Option<&List<Attestation<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTATIONS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.attestations),
            Self::Capella(inner) => Some(&inner.attestations),
            Self::Deneb(inner) => Some(&inner.attestations),
        }
    }
    pub fn attestations_mut(
        &mut self,
    ) -> Option<&mut List<Attestation<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTATIONS>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.attestations),
            Self::Capella(inner) => Some(&mut inner.attestations),
            Self::Deneb(inner) => Some(&mut inner.attestations),
        }
    }
    pub fn deposits(&self) -> Option<&List<Deposit, MAX_DEPOSITS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.deposits),
            Self::Capella(inner) => Some(&inner.deposits),
            Self::Deneb(inner) => Some(&inner.deposits),
        }
    }
    pub fn deposits_mut(&mut self) -> Option<&mut List<Deposit, MAX_DEPOSITS>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.deposits),
            Self::Capella(inner) => Some(&mut inner.deposits),
            Self::Deneb(inner) => Some(&mut inner.deposits),
        }
    }
    pub fn voluntary_exits(&self) -> Option<&List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.voluntary_exits),
            Self::Capella(inner) => Some(&inner.voluntary_exits),
            Self::Deneb(inner) => Some(&inner.voluntary_exits),
        }
    }
    pub fn voluntary_exits_mut(
        &mut self,
    ) -> Option<&mut List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.voluntary_exits),
            Self::Capella(inner) => Some(&mut inner.voluntary_exits),
            Self::Deneb(inner) => Some(&mut inner.voluntary_exits),
        }
    }
    pub fn sync_aggregate(&self) -> Option<&SyncAggregate<SYNC_COMMITTEE_SIZE>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.sync_aggregate),
            Self::Capella(inner) => Some(&inner.sync_aggregate),
            Self::Deneb(inner) => Some(&inner.sync_aggregate),
        }
    }
    pub fn sync_aggregate_mut(&mut self) -> Option<&mut SyncAggregate<SYNC_COMMITTEE_SIZE>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.sync_aggregate),
            Self::Capella(inner) => Some(&mut inner.sync_aggregate),
            Self::Deneb(inner) => Some(&mut inner.sync_aggregate),
        }
    }
    pub fn execution_payload_header(
        &self,
    ) -> Option<ExecutionPayloadHeaderRef<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>> {
        match self {
            Self::Bellatrix(inner) => Some(From::from(&inner.execution_payload_header)),
            Self::Capella(inner) => Some(From::from(&inner.execution_payload_header)),
            Self::Deneb(inner) => Some(From::from(&inner.execution_payload_header)),
        }
    }
    pub fn execution_payload_header_mut(
        &mut self,
    ) -> Option<ExecutionPayloadHeaderRefMut<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>> {
        match self {
            Self::Bellatrix(inner) => Some(From::from(&mut inner.execution_payload_header)),
            Self::Capella(inner) => Some(From::from(&mut inner.execution_payload_header)),
            Self::Deneb(inner) => Some(From::from(&mut inner.execution_payload_header)),
        }
    }
    pub fn bls_to_execution_changes(
        &self,
    ) -> Option<&List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&inner.bls_to_execution_changes),
        }
    }
    pub fn bls_to_execution_changes_mut(
        &mut self,
    ) -> Option<&mut List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&mut inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&mut inner.bls_to_execution_changes),
        }
    }
    pub fn blob_kzg_commitments(
        &self,
    ) -> Option<&List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.blob_kzg_commitments),
        }
    }
    pub fn blob_kzg_commitments_mut(
        &mut self,
    ) -> Option<&mut List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&mut inner.blob_kzg_commitments),
        }
    }
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
    > serde::Serialize
    for BlindedBeaconBlockBody<
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
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Bellatrix(inner) => <_ as serde::Serialize>::serialize(inner, serializer),
            Self::Capella(inner) => <_ as serde::Serialize>::serialize(inner, serializer),
            Self::Deneb(inner) => <_ as serde::Serialize>::serialize(inner, serializer),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum BlindedBeaconBlockBodyRef<
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
> {
    Bellatrix(
        &'a bellatrix::BlindedBeaconBlockBody<
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
        &'a capella::BlindedBeaconBlockBody<
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
        &'a deneb::BlindedBeaconBlockBody<
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
    >
    BlindedBeaconBlockBodyRef<
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
    >
{
    pub fn bellatrix(
        &self,
    ) -> Option<
        &bellatrix::BlindedBeaconBlockBody<
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
        &capella::BlindedBeaconBlockBody<
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
        &deneb::BlindedBeaconBlockBody<
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
    pub fn randao_reveal(&self) -> Option<&BlsSignature> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.randao_reveal),
            Self::Capella(inner) => Some(&inner.randao_reveal),
            Self::Deneb(inner) => Some(&inner.randao_reveal),
        }
    }
    pub fn eth1_data(&self) -> Option<&Eth1Data> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.eth1_data),
            Self::Capella(inner) => Some(&inner.eth1_data),
            Self::Deneb(inner) => Some(&inner.eth1_data),
        }
    }
    pub fn graffiti(&self) -> Option<&Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.graffiti),
            Self::Capella(inner) => Some(&inner.graffiti),
            Self::Deneb(inner) => Some(&inner.graffiti),
        }
    }
    pub fn proposer_slashings(&self) -> Option<&List<ProposerSlashing, MAX_PROPOSER_SLASHINGS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.proposer_slashings),
            Self::Capella(inner) => Some(&inner.proposer_slashings),
            Self::Deneb(inner) => Some(&inner.proposer_slashings),
        }
    }
    pub fn attester_slashings(
        &self,
    ) -> Option<&List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.attester_slashings),
            Self::Capella(inner) => Some(&inner.attester_slashings),
            Self::Deneb(inner) => Some(&inner.attester_slashings),
        }
    }
    pub fn attestations(
        &self,
    ) -> Option<&List<Attestation<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTATIONS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.attestations),
            Self::Capella(inner) => Some(&inner.attestations),
            Self::Deneb(inner) => Some(&inner.attestations),
        }
    }
    pub fn deposits(&self) -> Option<&List<Deposit, MAX_DEPOSITS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.deposits),
            Self::Capella(inner) => Some(&inner.deposits),
            Self::Deneb(inner) => Some(&inner.deposits),
        }
    }
    pub fn voluntary_exits(&self) -> Option<&List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.voluntary_exits),
            Self::Capella(inner) => Some(&inner.voluntary_exits),
            Self::Deneb(inner) => Some(&inner.voluntary_exits),
        }
    }
    pub fn sync_aggregate(&self) -> Option<&SyncAggregate<SYNC_COMMITTEE_SIZE>> {
        match self {
            Self::Bellatrix(inner) => Some(&inner.sync_aggregate),
            Self::Capella(inner) => Some(&inner.sync_aggregate),
            Self::Deneb(inner) => Some(&inner.sync_aggregate),
        }
    }
    pub fn execution_payload_header(
        &self,
    ) -> Option<ExecutionPayloadHeaderRef<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>> {
        match self {
            Self::Bellatrix(inner) => Some(From::from(&inner.execution_payload_header)),
            Self::Capella(inner) => Some(From::from(&inner.execution_payload_header)),
            Self::Deneb(inner) => Some(From::from(&inner.execution_payload_header)),
        }
    }
    pub fn bls_to_execution_changes(
        &self,
    ) -> Option<&List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&inner.bls_to_execution_changes),
        }
    }
    pub fn blob_kzg_commitments(
        &self,
    ) -> Option<&List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&inner.blob_kzg_commitments),
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
    >
    From<
        &'a bellatrix::BlindedBeaconBlockBody<
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
    for BlindedBeaconBlockBodyRef<
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
    >
{
    fn from(
        value: &'a bellatrix::BlindedBeaconBlockBody<
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
    >
    From<
        &'a capella::BlindedBeaconBlockBody<
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
    for BlindedBeaconBlockBodyRef<
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
    >
{
    fn from(
        value: &'a capella::BlindedBeaconBlockBody<
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
    >
    From<
        &'a deneb::BlindedBeaconBlockBody<
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
    for BlindedBeaconBlockBodyRef<
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
    >
{
    fn from(
        value: &'a deneb::BlindedBeaconBlockBody<
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
#[derive(Debug, PartialEq, Eq)]
pub enum BlindedBeaconBlockBodyRefMut<
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
> {
    Bellatrix(
        &'a mut bellatrix::BlindedBeaconBlockBody<
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
        &'a mut capella::BlindedBeaconBlockBody<
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
        &'a mut deneb::BlindedBeaconBlockBody<
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
    >
    BlindedBeaconBlockBodyRefMut<
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
    >
{
    pub fn bellatrix_mut(
        &mut self,
    ) -> Option<
        &mut bellatrix::BlindedBeaconBlockBody<
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
    pub fn capella_mut(
        &mut self,
    ) -> Option<
        &mut capella::BlindedBeaconBlockBody<
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
    pub fn deneb_mut(
        &mut self,
    ) -> Option<
        &mut deneb::BlindedBeaconBlockBody<
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
    pub fn randao_reveal_mut(&mut self) -> Option<&mut BlsSignature> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.randao_reveal),
            Self::Capella(inner) => Some(&mut inner.randao_reveal),
            Self::Deneb(inner) => Some(&mut inner.randao_reveal),
        }
    }
    pub fn eth1_data_mut(&mut self) -> Option<&mut Eth1Data> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.eth1_data),
            Self::Capella(inner) => Some(&mut inner.eth1_data),
            Self::Deneb(inner) => Some(&mut inner.eth1_data),
        }
    }
    pub fn graffiti_mut(&mut self) -> Option<&mut Bytes32> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.graffiti),
            Self::Capella(inner) => Some(&mut inner.graffiti),
            Self::Deneb(inner) => Some(&mut inner.graffiti),
        }
    }
    pub fn proposer_slashings_mut(
        &mut self,
    ) -> Option<&mut List<ProposerSlashing, MAX_PROPOSER_SLASHINGS>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.proposer_slashings),
            Self::Capella(inner) => Some(&mut inner.proposer_slashings),
            Self::Deneb(inner) => Some(&mut inner.proposer_slashings),
        }
    }
    pub fn attester_slashings_mut(
        &mut self,
    ) -> Option<&mut List<AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTER_SLASHINGS>>
    {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.attester_slashings),
            Self::Capella(inner) => Some(&mut inner.attester_slashings),
            Self::Deneb(inner) => Some(&mut inner.attester_slashings),
        }
    }
    pub fn attestations_mut(
        &mut self,
    ) -> Option<&mut List<Attestation<MAX_VALIDATORS_PER_COMMITTEE>, MAX_ATTESTATIONS>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.attestations),
            Self::Capella(inner) => Some(&mut inner.attestations),
            Self::Deneb(inner) => Some(&mut inner.attestations),
        }
    }
    pub fn deposits_mut(&mut self) -> Option<&mut List<Deposit, MAX_DEPOSITS>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.deposits),
            Self::Capella(inner) => Some(&mut inner.deposits),
            Self::Deneb(inner) => Some(&mut inner.deposits),
        }
    }
    pub fn voluntary_exits_mut(
        &mut self,
    ) -> Option<&mut List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.voluntary_exits),
            Self::Capella(inner) => Some(&mut inner.voluntary_exits),
            Self::Deneb(inner) => Some(&mut inner.voluntary_exits),
        }
    }
    pub fn sync_aggregate_mut(&mut self) -> Option<&mut SyncAggregate<SYNC_COMMITTEE_SIZE>> {
        match self {
            Self::Bellatrix(inner) => Some(&mut inner.sync_aggregate),
            Self::Capella(inner) => Some(&mut inner.sync_aggregate),
            Self::Deneb(inner) => Some(&mut inner.sync_aggregate),
        }
    }
    pub fn execution_payload_header_mut(
        &mut self,
    ) -> Option<ExecutionPayloadHeaderRefMut<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>> {
        match self {
            Self::Bellatrix(inner) => Some(From::from(&mut inner.execution_payload_header)),
            Self::Capella(inner) => Some(From::from(&mut inner.execution_payload_header)),
            Self::Deneb(inner) => Some(From::from(&mut inner.execution_payload_header)),
        }
    }
    pub fn bls_to_execution_changes_mut(
        &mut self,
    ) -> Option<&mut List<SignedBlsToExecutionChange, MAX_BLS_TO_EXECUTION_CHANGES>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(inner) => Some(&mut inner.bls_to_execution_changes),
            Self::Deneb(inner) => Some(&mut inner.bls_to_execution_changes),
        }
    }
    pub fn blob_kzg_commitments_mut(
        &mut self,
    ) -> Option<&mut List<KzgCommitment, MAX_BLOB_COMMITMENTS_PER_BLOCK>> {
        match self {
            Self::Bellatrix(_) => None,
            Self::Capella(_) => None,
            Self::Deneb(inner) => Some(&mut inner.blob_kzg_commitments),
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
    >
    From<
        &'a mut bellatrix::BlindedBeaconBlockBody<
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
    for BlindedBeaconBlockBodyRefMut<
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
    >
{
    fn from(
        value: &'a mut bellatrix::BlindedBeaconBlockBody<
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
    >
    From<
        &'a mut capella::BlindedBeaconBlockBody<
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
    for BlindedBeaconBlockBodyRefMut<
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
    >
{
    fn from(
        value: &'a mut capella::BlindedBeaconBlockBody<
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
    >
    From<
        &'a mut deneb::BlindedBeaconBlockBody<
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
    for BlindedBeaconBlockBodyRefMut<
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
    >
{
    fn from(
        value: &'a mut deneb::BlindedBeaconBlockBody<
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
