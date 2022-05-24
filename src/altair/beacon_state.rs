use crate::altair::sync::SyncCommittee;
use crate::phase0::beacon_block::BeaconBlockHeader;
use crate::phase0::fork::Fork;
use crate::phase0::operations::{Checkpoint, Eth1Data};
use crate::phase0::validator::Validator;
use crate::phase0::JUSTIFICATION_BITS_LENGTH;
use crate::primitives::{Bytes32, Gwei, Root, Slot};
use ssz_rs::prelude::*;

pub type ParticipationFlags = u8;

#[derive(Default, Debug, SimpleSerialize, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BeaconState<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
> {
    #[serde(with = "crate::serde::as_string")]
    pub genesis_time: u64,
    pub genesis_validators_root: Root,
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    pub fork: Fork,
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub state_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub historical_roots: List<Root, HISTORICAL_ROOTS_LIMIT>,
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: List<Eth1Data, ETH1_DATA_VOTES_BOUND>,
    #[serde(with = "crate::serde::as_string")]
    pub eth1_deposit_index: u64,
    pub validators: List<Validator, VALIDATOR_REGISTRY_LIMIT>,
    #[serde(with = "crate::serde::collection_over_string")]
    pub balances: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
    pub randao_mixes: Vector<Bytes32, EPOCHS_PER_HISTORICAL_VECTOR>,
    #[serde(with = "crate::serde::collection_over_string")]
    pub slashings: Vector<Gwei, EPOCHS_PER_SLASHINGS_VECTOR>,
    // TODO serde?
    pub previous_epoch_attestations: List<ParticipationFlags, VALIDATOR_REGISTRY_LIMIT>,
    // TODO serde?
    pub current_epoch_attestations: List<ParticipationFlags, VALIDATOR_REGISTRY_LIMIT>,
    pub justification_bits: Bitvector<JUSTIFICATION_BITS_LENGTH>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,
    // TODO fix serde
    pub inactivity_scores: List<u64, VALIDATOR_REGISTRY_LIMIT>,
    pub current_sync_committee: SyncCommittee<SYNC_COMMITTEE_SIZE>,
    pub next_sync_committee: SyncCommittee<SYNC_COMMITTEE_SIZE>,
}
