//! WARNING: This file was derived by the `gen-spec` utility. DO NOT EDIT MANUALLY.
use crate::altair as spec;
use crate::primitives::{Gwei, Hash32, GENESIS_EPOCH};
use crate::state_transition::{Context, Result};
use spec::{
    get_active_validator_indices, process_deposit, BeaconBlock, BeaconBlockBody,
    BeaconBlockHeader, BeaconState, Deposit, DepositData, Eth1Data, Fork,
    DEPOSIT_CONTRACT_TREE_DEPTH,
};
use ssz_rs::prelude::*;
pub use crate::altair::genesis::initialize_beacon_state_from_eth1;
const DEPOSIT_DATA_LIST_BOUND: usize = 2usize.pow(DEPOSIT_CONTRACT_TREE_DEPTH as u32);
pub fn is_valid_genesis_state<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    context: &Context,
) -> bool {
    if state.genesis_time < context.min_genesis_time {
        return false;
    }
    if get_active_validator_indices(state, GENESIS_EPOCH).len()
        < context.min_genesis_active_validator_count
    {
        return false;
    }
    true
}
pub fn get_genesis_block<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
>(
    genesis_state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
) -> Result<
    BeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
    >,
> {
    Ok(BeaconBlock {
        state_root: genesis_state.hash_tree_root()?,
        ..Default::default()
    })
}
