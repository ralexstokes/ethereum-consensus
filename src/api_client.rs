// TODO: remove once things are all used
#![allow(unused_variables)]

use crate::error::ApiError;
use crate::types::{
    ApiResult, AttestationDuty, BalanceSummary, BeaconHeaderSummary, BeaconProposerRegistration,
    BlockId, CommitteeDescriptor, CommitteeFilter, CommitteeSummary, EventTopic,
    FinalityCheckpoints, GenesisDetails, HealthStatus, NetworkIdentity, PeerDescription,
    PeerDescriptor, PeerSummary, ProposerDuty, PubkeyOrIndex, StateId, SyncCommitteeDescriptor,
    SyncCommitteeDuty, SyncCommitteeSummary, SyncStatus, ValidatorDescriptor, ValidatorSummary,
    Value, RootData
};
use ethereum_consensus::altair::mainnet::{
    SignedContributionAndProof, SyncCommitteeContribution, SyncCommitteeMessage,
};
use ethereum_consensus::bellatrix::mainnet::{BlindedBeaconBlock, SignedBlindedBeaconBlock};
use ethereum_consensus::builder::SignedValidatorRegistration;
use ethereum_consensus::networking::Multiaddr;
use ethereum_consensus::phase0::mainnet::{
    Attestation, AttestationData, AttesterSlashing, BeaconBlock, BeaconState, Fork,
    ProposerSlashing, SignedAggregateAndProof, SignedBeaconBlock, SignedVoluntaryExit,
};
use ethereum_consensus::primitives::{
    Bytes32, ChainId, CommitteeIndex, Coordinate, Epoch, ExecutionAddress, RandaoReveal, Root,
    Slot, ValidatorIndex,
};
use std::collections::HashMap;
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not parse URL: {0}")]
    Url(#[from] ParseError),
    #[error("could not send request: {0}")]
    Http(#[from] reqwest::Error),
    #[error("error from API: {0}")]
    Api(#[from] ApiError),
}

pub async fn api_error_or_ok(response: reqwest::Response) -> Result<(), Error> {
    if response.status() == reqwest::StatusCode::OK {
        Ok(())
    } else {
        let api_err = response.json::<ApiError>().await?;
        Err(Error::Api(api_err))
    }
}

pub struct Client {
    http: reqwest::Client,
    endpoint: Url,
}

impl Client {
    pub fn new_with_client<U: Into<Url>>(client: reqwest::Client, endpoint: U) -> Self {
        Self {
            http: client,
            endpoint: endpoint.into(),
        }
    }

    pub fn new<U: Into<Url>>(endpoint: U) -> Self {
        let client = reqwest::Client::new();
        Self::new_with_client(client, endpoint)
    }

    pub async fn get<T: serde::Serialize + serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, Error> {
        let result: ApiResult<T> = self.http_get(path).await?.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn http_get(&self, path: &str) -> Result<reqwest::Response, Error> {
        let target = self.endpoint.join(path)?;
        let response = self.http.get(target).send().await?;
        Ok(response)
    }

    pub async fn post<
        T: serde::Serialize + serde::de::DeserializeOwned,
        U: serde::Serialize + serde::de::DeserializeOwned,
    >(
        &self,
        path: &str,
        argument: &T,
    ) -> Result<U, Error> {
        let result: ApiResult<U> = self.http_post(path, argument).await?.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn http_post<T: serde::Serialize + serde::de::DeserializeOwned>(
        &self,
        path: &str,
        argument: &T,
    ) -> Result<reqwest::Response, Error> {
        let target = self.endpoint.join(path)?;
        let response = self.http.post(target).json(argument).send().await?;
        Ok(response)
    }

    /* beacon namespace */
    pub async fn get_genesis_details(&self) -> Result<GenesisDetails, Error> {
        self.get("/eth/v1/beacon/genesis").await
    }

    pub async fn get_state_root(&self, state_id: StateId) -> Result<Root, Error> {
        let mut stub = "";

        match state_id {
            StateId::Finalized => stub = "finalized",
            StateId::Head => stub = "head",
            StateId::Genesis => stub = "genesis",
            StateId::Root(_) => stub = "not implemented",
            StateId::Slot(_) => stub = "not implemented",
            StateId::Justified => stub = "justified",
        };

        let path = format!("eth/v1/beacon/states/{}/root", &stub);

        let root: Value<RootData> = self.get(&path).await?;

        Ok(root.data.root)
    }

    pub async fn get_fork(&self, state_id: StateId) -> Result<Fork, Error> {
        let mut stub = "";

        match state_id {
            StateId::Finalized => stub = "finalized",
            StateId::Head => stub = "head",
            StateId::Genesis => stub = "genesis",
            StateId::Root(_) => stub = "not implemented",
            StateId::Slot(_) => stub = "not implemented",
            StateId::Justified => stub = "justified",
        };

        let path = format!("eth/v1/beacon/states/{}/fork", &stub);
        let result: Value<Fork> = self.get(&path).await?;
        Ok(result.data)
    }

    pub async fn get_finality_checkpoints(id: StateId) -> Result<FinalityCheckpoints, Error> {
        unimplemented!("")
    }

    pub async fn get_validators(
        id: StateId,
        filters: &[ValidatorDescriptor],
    ) -> Result<Vec<ValidatorSummary>, Error> {
        unimplemented!("")
    }

    pub async fn get_validator(
        id: StateId,
        validator_id: PubkeyOrIndex,
    ) -> Result<ValidatorSummary, Error> {
        unimplemented!("")
    }

    pub async fn get_balances(
        id: StateId,
        filters: &[PubkeyOrIndex],
    ) -> Result<Vec<BalanceSummary>, Error> {
        unimplemented!("")
    }

    pub async fn get_all_committees(id: StateId) -> Result<Vec<CommitteeSummary>, Error> {
        unimplemented!("")
    }

    pub async fn get_committees(
        id: StateId,
        filter: CommitteeFilter,
    ) -> Result<Vec<CommitteeSummary>, Error> {
        unimplemented!("")
    }

    pub async fn get_sync_committees(
        id: StateId,
        epoch: Option<Epoch>,
    ) -> Result<Vec<SyncCommitteeSummary>, Error> {
        unimplemented!("")
    }

    pub async fn get_beacon_header_at_head() -> Result<BeaconHeaderSummary, Error> {
        unimplemented!("")
    }

    pub async fn get_beacon_header_for_slot(slot: Slot) -> Result<BeaconHeaderSummary, Error> {
        unimplemented!("")
    }

    pub async fn get_beacon_header_for_parent_root(
        parent_root: Root,
    ) -> Result<BeaconHeaderSummary, Error> {
        unimplemented!("")
    }

    pub async fn get_beacon_header(id: BlockId) -> Result<BeaconHeaderSummary, Error> {
        unimplemented!("")
    }

    pub async fn post_signed_beacon_block(block: &SignedBeaconBlock) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn post_signed_blinded_beacon_block(
        block: &SignedBlindedBeaconBlock,
    ) -> Result<(), Error> {
        unimplemented!("")
    }

    // v2 endpoint
    pub async fn get_beacon_block(id: BlockId) -> Result<SignedBeaconBlock, Error> {
        unimplemented!("")
    }

    pub async fn get_beacon_block_root(id: BlockId) -> Result<Root, Error> {
        unimplemented!("")
    }

    pub async fn get_attestations_from_beacon_block(
        id: BlockId,
    ) -> Result<Vec<Attestation>, Error> {
        unimplemented!("")
    }

    pub async fn get_attestations_from_pool(
        slot: Option<Slot>,
        committee_index: Option<CommitteeIndex>,
    ) -> Result<Vec<Attestation>, Error> {
        unimplemented!("")
    }

    pub async fn post_attestations(attestations: &[Attestation]) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn get_attester_slashings_from_pool() -> Result<Vec<AttesterSlashing>, Error> {
        unimplemented!("")
    }

    pub async fn post_attester_slashing(attester_slashing: &AttesterSlashing) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn get_proposer_slashings_from_pool() -> Result<Vec<ProposerSlashing>, Error> {
        unimplemented!("")
    }

    pub async fn post_proposer_slashing(proposer_slashing: &ProposerSlashing) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn post_sync_committee_messages(
        messages: &[SyncCommitteeMessage],
    ) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn get_voluntary_exits_from_pool() -> Result<Vec<SignedVoluntaryExit>, Error> {
        unimplemented!("")
    }

    pub async fn post_signed_voluntary_exit(exit: &SignedVoluntaryExit) -> Result<(), Error> {
        unimplemented!("")
    }

    /* config namespace */
    pub async fn get_fork_schedule() -> Result<Vec<Fork>, Error> {
        unimplemented!("")
    }

    pub async fn get_spec() -> Result<HashMap<String, String>, Error> {
        unimplemented!("")
    }

    pub async fn get_deposit_contract_address() -> Result<(ChainId, ExecutionAddress), Error> {
        unimplemented!("")
    }

    /* debug namespace */
    // v2 endpoint
    pub async fn get_state(id: StateId) -> Result<BeaconState, Error> {
        unimplemented!("")
    }

    // v2 endpoint
    pub async fn get_heads() -> Result<Vec<Coordinate>, Error> {
        unimplemented!("")
    }

    /* events namespace */
    // TODO: figure out return type
    pub async fn get_events<T>(topics: &[EventTopic]) -> Result<T, Error> {
        // get back "event: TOPIC, data: T"
        unimplemented!("")
    }

    /* node namespace */
    pub async fn get_node_identity() -> Result<NetworkIdentity, Error> {
        unimplemented!("")
    }

    pub async fn get_node_peers(filters: &[PeerDescriptor]) -> Result<Vec<PeerDescription>, Error> {
        unimplemented!("")
    }

    pub async fn get_peer(peer_id: Multiaddr) -> Result<PeerDescription, Error> {
        unimplemented!("")
    }

    pub async fn get_peer_count() -> Result<PeerSummary, Error> {
        unimplemented!("")
    }

    pub async fn get_node_version() -> Result<String, Error> {
        unimplemented!("")
    }

    pub async fn get_sync_status() -> Result<SyncStatus, Error> {
        unimplemented!("")
    }

    pub async fn get_health() -> Result<HealthStatus, Error> {
        unimplemented!("")
    }

    /* validator namespace */
    pub async fn get_attester_duties(
        epoch: Epoch,
        indices: &[ValidatorIndex],
    ) -> Result<(Root, Vec<AttestationDuty>), Error> {
        unimplemented!("")
    }

    pub async fn get_proposer_duties(epoch: Epoch) -> Result<(Root, Vec<ProposerDuty>), Error> {
        unimplemented!("")
    }

    pub async fn get_sync_committee_duties(
        epoch: Epoch,
        indices: &[ValidatorIndex],
    ) -> Result<(Root, Vec<SyncCommitteeDuty>), Error> {
        unimplemented!("")
    }

    // v2 endpoint
    pub async fn get_block(
        slot: Slot,
        randao_reveal: RandaoReveal,
        graffiti: Bytes32,
    ) -> Result<BeaconBlock, Error> {
        unimplemented!("")
    }

    pub async fn get_blinded_block(
        slot: Slot,
        randao_reveal: RandaoReveal,
        graffiti: Bytes32,
    ) -> Result<BlindedBeaconBlock, Error> {
        unimplemented!("")
    }

    pub async fn get_attestation_data(
        slot: Slot,
        committee_index: CommitteeIndex,
    ) -> Result<AttestationData, Error> {
        unimplemented!("")
    }

    pub async fn get_attestation_aggregate(
        attestation_data_root: Root,
        slot: Slot,
    ) -> Result<Attestation, Error> {
        unimplemented!("")
    }

    pub async fn post_aggregates_with_proofs(
        aggregates_with_proofs: &[SignedAggregateAndProof],
    ) -> Result<(), Error> {
        Ok(())
    }

    pub async fn subscribe_subnets_for_attestation(
        committee_descriptors: &[CommitteeDescriptor],
    ) -> Result<(), Error> {
        Ok(())
    }

    pub async fn subscribe_subnets_for_sync_committee(
        sync_committee_descriptors: &[SyncCommitteeDescriptor],
    ) -> Result<(), Error> {
        Ok(())
    }

    pub async fn get_sync_committee_contribution(
        slot: Slot,
        subcommittee_index: usize,
        beacon_block_root: Root,
    ) -> Result<SyncCommitteeContribution, Error> {
        unimplemented!("")
    }

    pub async fn post_contributions_with_proofs(
        contributions_with_proofs: &[SignedContributionAndProof],
    ) -> Result<(), Error> {
        Ok(())
    }

    pub async fn prepare_proposers(
        registrations: &[BeaconProposerRegistration],
    ) -> Result<(), Error> {
        Ok(())
    }

    // endpoint for builder registrations
    pub async fn register_validators_with_builders(
        registrations: &[SignedValidatorRegistration],
    ) -> Result<(), Error> {
        Ok(())
    }
}
