use crate::{
    phase0::{
        beacon_block::BeaconBlockHeader,
        constants::JUSTIFICATION_BITS_LENGTH,
        fork::Fork,
        operations::{Checkpoint, Eth1Data, PendingAttestation},
        validator::Validator,
    },
    primitives::{Bytes32, Gwei, Root, Slot},
};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HistoricalBatch<const SLOTS_PER_HISTORICAL_ROOT: usize> {
    pub block_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub state_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
}

// NOTE: `HistoricalSummary` is defined in the `capella` specs; however, this
// repo used the same strategy to compute the `HistoricalBatch` roots so
// the type already existed.
#[derive(Default, Debug, SimpleSerialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HistoricalSummary {
    pub block_summary_root: Root,
    pub state_summary_root: Root,
}

#[derive(Default, Debug, SimpleSerialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BeaconState<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
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
    pub previous_epoch_attestations:
        List<PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>, PENDING_ATTESTATIONS_BOUND>,
    pub current_epoch_attestations:
        List<PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>, PENDING_ATTESTATIONS_BOUND>,
    pub justification_bits: Bitvector<JUSTIFICATION_BITS_LENGTH>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,
}
