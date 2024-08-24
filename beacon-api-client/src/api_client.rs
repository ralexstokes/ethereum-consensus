use crate::{
    types::{
        ApiResult, AttestationDuty, BalanceSummary, BeaconHeaderSummary,
        BeaconProposerRegistration, BlockId, BroadcastValidation, CommitteeDescriptor,
        CommitteeFilter, CommitteeSummary, ConnectionOrientation, CoordinateWithMetadata,
        DepositContract, DepositSnapshot, FinalityCheckpoints, GenesisDetails, HealthStatus,
        NetworkIdentity, PeerDescription, PeerState, PeerSummary, ProposerDuty, PublicKeyOrIndex,
        RootData, StateId, SubmitSignedBeaconBlock, SyncCommitteeDescriptor, SyncCommitteeDuty,
        SyncCommitteeSummary, SyncStatus, Topic, ValidatorLiveness, ValidatorStatus,
        ValidatorSummary, Value, VersionData, VersionedValue,
    },
    ApiError, Error,
};
use ethereum_consensus::{
    altair::SyncCommitteeMessage,
    builder::SignedValidatorRegistration,
    capella::{SignedBlsToExecutionChange, Withdrawal},
    deneb::{
        self,
        mainnet::{
            BYTES_PER_LOGS_BLOOM, EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR,
            ETH1_DATA_VOTES_BOUND, HISTORICAL_ROOTS_LIMIT, MAX_EXTRA_DATA_BYTES,
            MAX_VALIDATORS_PER_COMMITTEE, SLOTS_PER_HISTORICAL_ROOT, SYNC_COMMITTEE_SIZE,
            VALIDATOR_REGISTRY_LIMIT,
        },
        minimal::BeaconState,
    },
    networking::PeerId,
    phase0::{AttestationData, Fork, ProposerSlashing, SignedVoluntaryExit},
    primitives::{
        BlobIndex, Bytes32, CommitteeIndex, Epoch, RandaoReveal, Root, Slot, ValidatorIndex,
    },
    ssz::prelude::Deserialize,
    Fork as Version,
};
use http::StatusCode;
use itertools::Itertools;
use mev_share_sse::{client::EventStream, EventClient};
use std::collections::HashMap;
use url::Url;

pub const CONSENSUS_VERSION_HEADER: &str = "eth-consensus-version";

pub async fn api_error_or_ok(response: reqwest::Response) -> Result<(), Error> {
    match response.status() {
        reqwest::StatusCode::OK | reqwest::StatusCode::ACCEPTED => Ok(()),
        _ => {
            let api_err = response.json::<ApiError>().await?;
            Err(Error::Api(api_err))
        }
    }
}

async fn api_error_or_value<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, Error> {
    match response.status() {
        reqwest::StatusCode::OK | reqwest::StatusCode::ACCEPTED => {
            let value = response.json().await?;
            Ok(value)
        }
        _ => {
            let api_err = response.json::<ApiError>().await?;
            Err(Error::Api(api_err))
        }
    }
}

pub trait ClientTypes: Clone {
    type SignedContributionAndProof: serde::Serialize;
    type SyncCommitteeContribution: serde::Serialize + serde::de::DeserializeOwned;
    type BlindedBeaconBlock: serde::Serialize + serde::de::DeserializeOwned;
    type SignedBlindedBeaconBlock: serde::Serialize + serde::de::DeserializeOwned;
    type Attestation: serde::Serialize + serde::de::DeserializeOwned;
    type AttesterSlashing: serde::Serialize + serde::de::DeserializeOwned;
    type BeaconBlock: serde::Serialize + serde::de::DeserializeOwned;
    type BeaconState: serde::Serialize + serde::de::DeserializeOwned;
    type SignedAggregateAndProof: serde::Serialize;
    type SignedBeaconBlock: serde::Serialize + serde::de::DeserializeOwned;
    type Blob: serde::Serialize + serde::de::DeserializeOwned;
    type BlobSidecar: serde::Serialize + serde::de::DeserializeOwned;
    type LightClientBootstrap: serde::Serialize + serde::de::DeserializeOwned;
    type LightClientUpdate: serde::Serialize + serde::de::DeserializeOwned;
    type LightClientFinalityUpdate: serde::Serialize + serde::de::DeserializeOwned;
    type LightClientOptimisticUpdate: serde::Serialize + serde::de::DeserializeOwned;
}

#[derive(Clone)]
pub struct Client<C> {
    pub http: reqwest::Client,
    pub endpoint: Url,
    _phantom: std::marker::PhantomData<C>,
}

impl<C: ClientTypes> Client<C> {
    pub fn new_with_client<U: Into<Url>>(client: reqwest::Client, endpoint: U) -> Self {
        Self { http: client, endpoint: endpoint.into(), _phantom: std::marker::PhantomData }
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

    pub async fn post<T: serde::Serialize + ?Sized>(
        &self,
        path: &str,
        argument: &T,
    ) -> Result<(), Error> {
        let response = self.http_post(path, argument).await?;
        api_error_or_ok(response).await
    }

    pub async fn http_post<T: serde::Serialize + ?Sized>(
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
        let details: Value<GenesisDetails> = self.get("eth/v1/beacon/genesis").await?;
        Ok(details.data)
    }

    pub async fn get_state_root(&self, state_id: StateId) -> Result<Root, Error> {
        let path = format!("eth/v1/beacon/states/{state_id}/root");
        let root: Value<RootData> = self.get(&path).await?;
        Ok(root.data.root)
    }

    pub async fn get_fork(&self, state_id: StateId) -> Result<Fork, Error> {
        let path = format!("eth/v1/beacon/states/{state_id}/fork");
        let result: Value<Fork> = self.get(&path).await?;
        Ok(result.data)
    }

    pub async fn get_finality_checkpoints(
        &self,
        id: StateId,
    ) -> Result<FinalityCheckpoints, Error> {
        let path = format!("eth/v1/beacon/states/{id}/finality_checkpoints");
        let result: Value<FinalityCheckpoints> = self.get(&path).await?;
        Ok(result.data)
    }

    pub async fn get_validators(
        &self,
        state_id: StateId,
        validator_ids: &[PublicKeyOrIndex],
        filters: &[ValidatorStatus],
    ) -> Result<Vec<ValidatorSummary>, Error> {
        let path = format!("eth/v1/beacon/states/{state_id}/validators");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        if !validator_ids.is_empty() {
            let validator_ids = validator_ids.iter().join(",");
            request = request.query(&[("id", validator_ids)]);
        }
        if !filters.is_empty() {
            let filters = filters.iter().join(",");
            request = request.query(&[("status", filters)]);
        }
        let response = request.send().await?;

        let result: ApiResult<Value<Vec<ValidatorSummary>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_validator(
        &self,
        state_id: StateId,
        validator_id: PublicKeyOrIndex,
    ) -> Result<ValidatorSummary, Error> {
        let path = format!("eth/v1/beacon/states/{state_id}/validators/{validator_id}");
        let result: Value<ValidatorSummary> = self.get(&path).await?;
        Ok(result.data)
    }

    pub async fn get_balances(
        &self,
        id: StateId,
        filters: &[PublicKeyOrIndex],
    ) -> Result<Vec<BalanceSummary>, Error> {
        let path = format!("eth/v1/beacon/states/{id}/validator_balances");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);

        if !filters.is_empty() {
            let filters = filters.iter().join(",");
            request = request.query(&[("id", filters)]);
        }
        let response = request.send().await?;

        let result: ApiResult<Value<Vec<BalanceSummary>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_all_committees(&self, id: StateId) -> Result<Vec<CommitteeSummary>, Error> {
        self.get_committees(id, CommitteeFilter::default()).await
    }

    pub async fn get_committees(
        &self,
        id: StateId,
        filter: CommitteeFilter,
    ) -> Result<Vec<CommitteeSummary>, Error> {
        let path = format!("eth/v1/beacon/states/{id}/committees");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        if let Some(epoch) = filter.epoch {
            request = request.query(&[("epoch", epoch)]);
        }
        if let Some(index) = filter.index {
            request = request.query(&[("index", index)]);
        }
        if let Some(slot) = filter.slot {
            request = request.query(&[("slot", slot)]);
        }
        let response = request.send().await?;
        let result: ApiResult<Value<Vec<CommitteeSummary>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_sync_committees(
        &self,
        id: StateId,
        epoch: Option<Epoch>,
    ) -> Result<SyncCommitteeSummary, Error> {
        let path = format!("eth/v1/beacon/states/{id}/sync_committees");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        if let Some(epoch) = epoch {
            request = request.query(&[("epoch", epoch)]);
        }
        let response = request.send().await?;
        let result: ApiResult<Value<SyncCommitteeSummary>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_randao(&self, id: StateId, epoch: Option<Epoch>) -> Result<Bytes32, Error> {
        let path = format!("eth/v1/beacon/states/{id}/randao");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        if let Some(epoch) = epoch {
            request = request.query(&[("epoch", epoch)]);
        }
        let response = request.send().await?;

        let result: ApiResult<Value<Bytes32>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_beacon_header_at_head(&self) -> Result<BeaconHeaderSummary, Error> {
        let result: Value<BeaconHeaderSummary> = self.get("eth/v1/beacon/headers").await?;
        Ok(result.data)
    }

    pub async fn get_beacon_header_for_slot(
        &self,
        slot: Slot,
    ) -> Result<BeaconHeaderSummary, Error> {
        let target = self.endpoint.join("eth/v1/beacon/headers")?;
        let mut request = self.http.get(target);
        request = request.query(&[("slot", slot)]);
        let response = request.send().await?;
        let result: ApiResult<Value<BeaconHeaderSummary>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_beacon_header_for_parent_root(
        &self,
        parent_root: Root,
    ) -> Result<BeaconHeaderSummary, Error> {
        let target = self.endpoint.join("eth/v1/beacon/headers")?;
        let mut request = self.http.get(target);
        request = request.query(&[("parent_root", format!("{parent_root:?}"))]);
        let response = request.send().await?;
        let result: ApiResult<Value<BeaconHeaderSummary>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_beacon_header(&self, id: BlockId) -> Result<BeaconHeaderSummary, Error> {
        let path = format!("eth/v1/beacon/headers/{id}");
        let result: Value<BeaconHeaderSummary> = self.get(&path).await?;
        Ok(result.data)
    }

    pub async fn post_signed_blinded_beacon_block(
        &self,
        block: &C::SignedBlindedBeaconBlock,
    ) -> Result<(), Error> {
        self.post("eth/v1/beacon/blinded_blocks", block).await
    }

    pub async fn post_signed_blinded_beacon_block_v2(
        &self,
        block: &C::SignedBlindedBeaconBlock,
        version: Version,
        broadcast_validation: Option<BroadcastValidation>,
    ) -> Result<(), Error> {
        let target = self.endpoint.join("eth/v2/beacon/blinded_blocks")?;
        let mut request = self
            .http
            .post(target)
            .json(block)
            .header(CONSENSUS_VERSION_HEADER, version.to_string());
        if let Some(validation) = broadcast_validation {
            request = request.query(&[("broadcast_validation", validation)]);
        }
        let response = request.send().await?;
        api_error_or_ok(response).await
    }

    pub async fn post_signed_beacon_block(
        &self,
        block: &C::SignedBeaconBlock,
    ) -> Result<(), Error> {
        self.post("eth/v1/beacon/blocks", block).await
    }

    // NOTE: this only supports the `deneb` fork at the moment
    // The message schema of `request` is different before `deneb`.
    pub async fn post_signed_beacon_block_v2(
        &self,
        request: SubmitSignedBeaconBlock<'_, C::SignedBeaconBlock, C::Blob>,
        version: Version,
        broadcast_validation: Option<BroadcastValidation>,
    ) -> Result<(), Error> {
        let target = self.endpoint.join("eth/v2/beacon/blocks")?;
        let mut request = self
            .http
            .post(target)
            .json(&request)
            .header(CONSENSUS_VERSION_HEADER, version.to_string());
        if let Some(validation) = broadcast_validation {
            request = request.query(&[("broadcast_validation", validation)]);
        }
        let response = request.send().await?;
        api_error_or_ok(response).await
    }

    // v2 endpoint
    pub async fn get_beacon_block(&self, id: BlockId) -> Result<C::SignedBeaconBlock, Error> {
        let result: VersionedValue<C::SignedBeaconBlock> =
            self.get(&format!("eth/v2/beacon/blocks/{id}")).await?;
        Ok(result.data)
    }

    pub async fn get_beacon_block_root(&self, id: BlockId) -> Result<Root, Error> {
        let result: Value<RootData> = self.get(&format!("eth/v1/beacon/blocks/{id}/root")).await?;
        Ok(result.data.root)
    }

    pub async fn get_attestations_from_beacon_block(
        &self,
        id: BlockId,
    ) -> Result<Vec<C::Attestation>, Error> {
        let result: Value<Vec<C::Attestation>> =
            self.get(&format!("eth/v1/beacon/blocks/{id}/attestations")).await?;
        Ok(result.data)
    }

    pub async fn get_blob_sidecars(
        &self,
        id: BlockId,
        indices: &[BlobIndex],
    ) -> Result<Vec<C::BlobSidecar>, Error> {
        let path = format!("eth/v1/beacon/blob_sidecars/{id}");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        if !indices.is_empty() {
            request = request.query(&[("indices", indices)]);
        }
        let response = request.send().await?;
        let result: ApiResult<Value<_>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_deposit_snapshot(&self) -> Result<DepositSnapshot, Error> {
        let result: Value<DepositSnapshot> = self.get("eth/v1/beacon/deposit_snapshot").await?;
        Ok(result.data)
    }

    pub async fn get_blinded_block(
        &self,
        id: BlockId,
    ) -> Result<C::SignedBlindedBeaconBlock, Error> {
        let result: VersionedValue<C::SignedBlindedBeaconBlock> =
            self.get(&format!("eth/v1/beacon/blinded_blocks/{id}")).await?;
        Ok(result.data)
    }

    pub async fn get_light_client_bootstrap(
        &self,
        block: Root,
    ) -> Result<C::LightClientBootstrap, Error> {
        let result: Value<_> =
            self.get(&format!("eth/v1/beacon/light_client/bootstrap/{block:?}")).await?;
        Ok(result.data)
    }

    pub async fn get_light_client_updates(
        &self,
        start: u64,
        count: u64,
    ) -> Result<Vec<C::LightClientUpdate>, Error> {
        let target = self.endpoint.join("eth/v1/beacon/light_client/updates")?;
        let mut request = self.http.get(target);
        request = request.query(&[("start_period", start), ("count", count)]);

        let response = request.send().await?;
        let result: ApiResult<Value<_>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_light_client_finality_update(
        &self,
    ) -> Result<C::LightClientFinalityUpdate, Error> {
        let result: Value<_> = self.get("eth/v1/beacon/light_client/finality_update").await?;
        Ok(result.data)
    }

    pub async fn get_light_client_optimistic_update(
        &self,
    ) -> Result<C::LightClientOptimisticUpdate, Error> {
        let result: Value<_> = self.get("eth/v1/beacon/light_client/optimistic_update").await?;
        Ok(result.data)
    }

    pub async fn get_attestations_from_pool(
        &self,
        slot: Option<Slot>,
        committee_index: Option<CommitteeIndex>,
    ) -> Result<Vec<C::Attestation>, Error> {
        let path = "eth/v1/beacon/pool/attestations";
        let target = self.endpoint.join(path)?;
        let mut request = self.http.get(target);
        if let Some(slot) = slot {
            request = request.query(&[("slot", slot)]);
        }
        if let Some(committee_index) = committee_index {
            request = request.query(&[("committee_index", committee_index)]);
        }
        let response = request.send().await?;
        let result: ApiResult<Value<Vec<C::Attestation>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn post_attestations(&self, attestations: &[C::Attestation]) -> Result<(), Error> {
        self.post("eth/v1/beacon/pool/attestations", attestations).await
    }

    pub async fn get_attester_slashings_from_pool(
        &self,
    ) -> Result<Vec<C::AttesterSlashing>, Error> {
        let result: Value<Vec<C::AttesterSlashing>> =
            self.get("eth/v1/beacon/pool/attester_slashings").await?;
        Ok(result.data)
    }

    pub async fn post_attester_slashing(
        &self,
        attester_slashing: &C::AttesterSlashing,
    ) -> Result<(), Error> {
        self.post("eth/v1/beacon/pool/attester_slashings", attester_slashing).await
    }

    pub async fn get_proposer_slashings_from_pool(&self) -> Result<Vec<ProposerSlashing>, Error> {
        let result: Value<Vec<ProposerSlashing>> =
            self.get("eth/v1/beacon/pool/proposer_slashings").await?;
        Ok(result.data)
    }

    pub async fn post_proposer_slashing(
        &self,
        proposer_slashing: &ProposerSlashing,
    ) -> Result<(), Error> {
        self.post("eth/v1/beacon/pool/proposer_slashings", proposer_slashing).await
    }

    pub async fn post_sync_committee_messages(
        &self,
        messages: &[SyncCommitteeMessage],
    ) -> Result<(), Error> {
        self.post("eth/v1/beacon/pool/sync_committees", messages).await
    }

    pub async fn get_voluntary_exits_from_pool(&self) -> Result<Vec<SignedVoluntaryExit>, Error> {
        let result: Value<Vec<SignedVoluntaryExit>> =
            self.get("eth/v1/beacon/pool/voluntary_exits").await?;
        Ok(result.data)
    }

    pub async fn post_signed_voluntary_exit(
        &self,
        exit: &SignedVoluntaryExit,
    ) -> Result<(), Error> {
        self.post("eth/v1/beacon/pool/voluntary_exits", exit).await
    }

    pub async fn get_bls_to_execution_changes(
        &self,
    ) -> Result<Vec<SignedBlsToExecutionChange>, Error> {
        let result: Value<Vec<SignedBlsToExecutionChange>> =
            self.get("eth/v1/beacon/pool/bls_to_execution_changes").await?;
        Ok(result.data)
    }

    pub async fn post_bls_to_execution_changes(
        &self,
        changes: &[SignedBlsToExecutionChange],
    ) -> Result<(), Error> {
        self.post("eth/v1/beacon/pool/bls_to_execution_changes", changes).await
    }

    /* builder namespace */
    pub async fn get_expected_withdrawals(
        &self,
        id: StateId,
        slot: Option<Slot>,
    ) -> Result<Vec<Withdrawal>, Error> {
        let path = format!("eth/v1/builder/states/{id}/expected_withdrawals");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        if let Some(slot) = slot {
            request = request.query(&[("proposal_slot", slot)]);
        }

        let response = request.send().await?;
        let result: ApiResult<Value<_>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    /* config namespace */
    pub async fn get_fork_schedule(&self) -> Result<Vec<Fork>, Error> {
        let result: Value<Vec<Fork>> = self.get("eth/v1/config/fork_schedule").await?;
        Ok(result.data)
    }

    pub async fn get_spec(&self) -> Result<HashMap<String, String>, Error> {
        let result: Value<HashMap<String, String>> = self.get("eth/v1/config/spec").await?;
        Ok(result.data)
    }

    pub async fn get_deposit_contract_address(&self) -> Result<DepositContract, Error> {
        let result: Value<DepositContract> = self.get("eth/v1/config/deposit_contract").await?;
        Ok(result.data)
    }

    /* debug namespace */
    // v2 endpoint
    pub async fn get_state(&self, id: StateId) -> Result<C::BeaconState, Error> {
        let result: VersionedValue<C::BeaconState> =
            self.get(&format!("eth/v2/debug/beacon/states/{id}")).await?;
        Ok(result.data)
    }

    pub async fn get_state_raw(&self, id: StateId) -> Result<C::BeaconState, Error> {
        let path = format!("eth/v2/debug/beacon/states/{id}");
        let target = self.endpoint.join(&path)?;
        let response =
            self.http.get(target).header("Accept", "application/octet-stream").send().await?;

        let a = response.bytes().await?;
        let b: &[u8] = &a;
        // println!("b: {:?}", b);
        // println!(
        //     "b: {:?}",
        //     deneb::BeaconState::<
        //         SLOTS_PER_HISTORICAL_ROOT,
        //         HISTORICAL_ROOTS_LIMIT,
        //         ETH1_DATA_VOTES_BOUND,
        //         VALIDATOR_REGISTRY_LIMIT,
        //         EPOCHS_PER_HISTORICAL_VECTOR,
        //         EPOCHS_PER_SLASHINGS_VECTOR,
        //         MAX_VALIDATORS_PER_COMMITTEE,
        //         SYNC_COMMITTEE_SIZE,
        //         BYTES_PER_LOGS_BLOOM,
        //         MAX_EXTRA_DATA_BYTES,
        //     >::deserialize(b)

        let c = deneb::BeaconState::<
            SLOTS_PER_HISTORICAL_ROOT,
            HISTORICAL_ROOTS_LIMIT,
            ETH1_DATA_VOTES_BOUND,
            VALIDATOR_REGISTRY_LIMIT,
            EPOCHS_PER_HISTORICAL_VECTOR,
            EPOCHS_PER_SLASHINGS_VECTOR,
            MAX_VALIDATORS_PER_COMMITTEE,
            SYNC_COMMITTEE_SIZE,
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
        >::deserialize(b)
        .unwrap();

        println!("c.slot: {:?}", c.slot);

        // );

        return Err(Error::MissingExpectedData("Asd".into()));
    }

    // pub async fn get<T: serde::Serialize + serde::de::DeserializeOwned>(
    //     &self,
    //     path: &str,
    // ) -> Result<T, Error> {
    //     let result: ApiResult<T> = self.http_get(path).await?.json().await?;
    //     match result {
    //         ApiResult::Ok(result) => Ok(result),
    //         ApiResult::Err(err) => Err(err.into()),
    //     }
    // }

    // pub async fn http_get(&self, path: &str) -> Result<reqwest::Response, Error> {
    //     let target = self.endpoint.join(path)?;
    //     let response = self.http.get(target).send().await?;
    //     Ok(response)
    // }

    // v2 endpoint
    pub async fn get_heads(&self) -> Result<Vec<CoordinateWithMetadata>, Error> {
        let result: Value<Vec<CoordinateWithMetadata>> =
            self.get("eth/v2/debug/beacon/heads").await?;
        Ok(result.data)
    }

    /* events namespace */
    pub async fn get_events<T: Topic>(&self) -> Result<EventStream<T::Data>, Error> {
        let name = T::NAME;
        let path = format!("/eth/v1/events?topics={name}");
        let target = self.endpoint.join(&path)?.to_string();

        let client = EventClient::default();
        Ok(client.subscribe(&target).await?)
    }

    /* node namespace */
    pub async fn get_node_identity(&self) -> Result<NetworkIdentity, Error> {
        let result: Value<NetworkIdentity> = self.get("eth/v1/node/identity").await?;
        Ok(result.data)
    }

    pub async fn get_node_peers(
        &self,
        peer_states: &[PeerState],
        connection_orientations: &[ConnectionOrientation],
    ) -> Result<Vec<PeerDescription>, Error> {
        let path = "eth/v1/node/peers";
        let target = self.endpoint.join(path)?;
        let mut request = self.http.get(target);
        if !peer_states.is_empty() {
            request = request.query(&[("state", peer_states.iter().join(","))]);
        }
        if !connection_orientations.is_empty() {
            request = request.query(&[("direction", connection_orientations.iter().join(","))]);
        }
        let response = request.send().await?;
        let result: ApiResult<Value<Vec<PeerDescription>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_peer(&self, peer_id: PeerId) -> Result<PeerDescription, Error> {
        let result: Value<PeerDescription> =
            self.get(&format!("/eth/v1/node/peers/{peer_id}")).await?;
        Ok(result.data)
    }

    pub async fn get_peer_summary(&self) -> Result<PeerSummary, Error> {
        let result: Value<PeerSummary> = self.get("eth/v1/node/peer_count").await?;
        Ok(result.data)
    }

    pub async fn get_node_version(&self) -> Result<String, Error> {
        let result: Value<VersionData> = self.get("eth/v1/node/version").await?;
        Ok(result.data.version)
    }

    pub async fn get_sync_status(&self) -> Result<SyncStatus, Error> {
        let result: Value<SyncStatus> = self.get("eth/v1/node/syncing").await?;
        Ok(result.data)
    }

    pub async fn get_health(&self) -> Result<HealthStatus, Error> {
        let path = "eth/v1/node/health";
        let target = self.endpoint.join(path)?;
        let request = self.http.get(target);
        let response = request.send().await?;
        let result = match response.status() {
            StatusCode::OK => HealthStatus::Ready,
            StatusCode::PARTIAL_CONTENT => HealthStatus::Syncing,
            StatusCode::SERVICE_UNAVAILABLE => HealthStatus::NotInitialized,
            _ => HealthStatus::Unknown,
        };
        Ok(result)
    }

    /* validator namespace */
    pub async fn get_attester_duties(
        &self,
        epoch: Epoch,
        indices: &[ValidatorIndex],
    ) -> Result<(Root, Vec<AttestationDuty>), Error> {
        let endpoint = format!("eth/v1/validator/duties/attester/{epoch}");
        let indices = indices.iter().map(|index| index.to_string()).collect::<Vec<_>>();
        let response = self.http_post(&endpoint, &indices).await?;
        let mut result: Value<Vec<AttestationDuty>> = api_error_or_value(response).await?;
        let dependent_root_value = result
            .meta
            .remove("dependent_root")
            .ok_or_else(|| Error::MissingExpectedData("`dependent_root`".to_string()))?;
        let dependent_root: Root = serde_json::from_value(dependent_root_value)?;
        Ok((dependent_root, result.data))
    }

    pub async fn get_proposer_duties(
        &self,
        epoch: Epoch,
    ) -> Result<(Root, Vec<ProposerDuty>), Error> {
        let endpoint = format!("eth/v1/validator/duties/proposer/{epoch}");
        let mut result: Value<Vec<ProposerDuty>> = self.get(&endpoint).await?;
        let dependent_root_value = result.meta.remove("dependent_root").ok_or_else(|| {
            Error::MissingExpectedData("missing `dependent_root` in response".to_string())
        })?;
        let dependent_root: Root = serde_json::from_value(dependent_root_value)?;
        Ok((dependent_root, result.data))
    }

    pub async fn get_sync_committee_duties(
        &self,
        epoch: Epoch,
        indices: &[ValidatorIndex],
    ) -> Result<Vec<SyncCommitteeDuty>, Error> {
        let endpoint = format!("eth/v1/validator/duties/sync/{epoch}");
        let indices = indices.iter().map(|index| index.to_string()).collect::<Vec<_>>();
        let response = self.http_post(&endpoint, &indices).await?;
        let result: Value<Vec<SyncCommitteeDuty>> = api_error_or_value(response).await?;
        Ok(result.data)
    }

    // v2 endpoint
    pub async fn get_block_proposal(
        &self,
        slot: Slot,
        randao_reveal: RandaoReveal,
        graffiti: Option<Bytes32>,
    ) -> Result<C::BeaconBlock, Error> {
        let path = format!("eth/v2/validator/blocks/{slot}");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        request = request.query(&[("randao_reveal", format!("{randao_reveal:?}"))]);
        if let Some(graffiti) = graffiti {
            request = request.query(&[("graffiti", format!("{graffiti:?}"))]);
        }
        let response = request.send().await?;
        let result: ApiResult<VersionedValue<C::BeaconBlock>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_blinded_block_proposal(
        &self,
        slot: Slot,
        randao_reveal: RandaoReveal,
        graffiti: Option<Bytes32>,
    ) -> Result<C::BlindedBeaconBlock, Error> {
        let path = format!("eth/v1/validator/blinded_blocks/{slot}");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        request = request.query(&[("randao_reveal", format!("{randao_reveal:?}"))]);
        if let Some(graffiti) = graffiti {
            request = request.query(&[("graffiti", format!("{graffiti:?}"))]);
        }
        let response = request.send().await?;
        let result: ApiResult<VersionedValue<C::BlindedBeaconBlock>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_attestation_data(
        &self,
        slot: Slot,
        committee_index: CommitteeIndex,
    ) -> Result<AttestationData, Error> {
        let target = self.endpoint.join("eth/v1/validator/attestation_data")?;
        let mut request = self.http.get(target);
        request = request.query(&[("slot", slot)]);
        request = request.query(&[("committee_index", committee_index)]);
        let response = request.send().await?;
        let result: ApiResult<Value<AttestationData>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_attestation_aggregate(
        &self,
        attestation_data_root: Root,
        slot: Slot,
    ) -> Result<C::Attestation, Error> {
        let target = self.endpoint.join("eth/v1/validator/aggregate_attestation")?;
        let mut request = self.http.get(target);
        request = request.query(&[("attestation_data_root", format!("{attestation_data_root:?}"))]);
        request = request.query(&[("slot", slot)]);
        let response = request.send().await?;
        let result: ApiResult<Value<C::Attestation>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn post_aggregates_with_proofs(
        &self,
        aggregates_with_proofs: &[C::SignedAggregateAndProof],
    ) -> Result<(), Error> {
        self.post("eth/v1/validator/aggregate_and_proofs", aggregates_with_proofs).await
    }

    pub async fn subscribe_subnets_for_attestation_committees(
        &self,
        committee_descriptors: &[CommitteeDescriptor],
    ) -> Result<(), Error> {
        self.post("eth/v1/validator/beacon_committee_subscriptions", committee_descriptors).await
    }

    pub async fn subscribe_subnets_for_sync_committees(
        &self,
        sync_committee_descriptors: &[SyncCommitteeDescriptor],
    ) -> Result<(), Error> {
        self.post("eth/v1/validator/sync_committee_subscriptions", sync_committee_descriptors).await
    }

    pub async fn get_sync_committee_contribution(
        &self,
        slot: Slot,
        subcommittee_index: usize,
        beacon_block_root: Root,
    ) -> Result<C::SyncCommitteeContribution, Error> {
        let target = self.endpoint.join("eth/v1/validator/sync_committee_contribution")?;
        let mut request = self.http.get(target);
        request = request.query(&[("slot", slot)]);
        request = request.query(&[("subcommittee_index", subcommittee_index)]);
        request = request.query(&[("beacon_block_root", format!("{beacon_block_root:?}"))]);
        let response = request.send().await?;
        let result: ApiResult<Value<C::SyncCommitteeContribution>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn post_sync_committee_contributions_with_proofs(
        &self,
        contributions_with_proofs: &[C::SignedContributionAndProof],
    ) -> Result<(), Error> {
        self.post("eth/v1/validator/contribution_and_proofs", contributions_with_proofs).await
    }

    pub async fn prepare_proposers(
        &self,
        registrations: &[BeaconProposerRegistration],
    ) -> Result<(), Error> {
        self.post("eth/v1/validator/prepare_beacon_proposer", registrations).await
    }

    // endpoint for builder registrations
    pub async fn register_validators_with_builders(
        &self,
        registrations: &[SignedValidatorRegistration],
    ) -> Result<(), Error> {
        self.post("eth/v1/validator/register_validator", registrations).await
    }

    pub async fn post_liveness(
        &self,
        epoch: Epoch,
        indices: &[ValidatorIndex],
    ) -> Result<Vec<ValidatorLiveness>, Error> {
        let response =
            self.http_post(&format!("eth/v1/validator/liveness/{epoch}"), indices).await?;
        let result: ApiResult<Value<_>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }
}
