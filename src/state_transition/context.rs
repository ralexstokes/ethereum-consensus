use crate::altair::beacon_state::{self as altair, ParticipationFlags};
use crate::altair::sync::SyncCommittee;
use crate::phase0::beacon_block::BeaconBlockHeader;
use crate::phase0::beacon_state as phase0;
use crate::phase0::fork::Fork;
use crate::phase0::operations::{Checkpoint, Eth1Data, PendingAttestation};
use crate::phase0::validator::Validator;
use crate::phase0::JUSTIFICATION_BITS_LENGTH;
use crate::primitives::{Bytes32, Gwei, Root, Slot};
use crate::state_transition::spec::{ForkSchedule, Spec};
use crate::state_transition::types::BeaconState;
use crate::state_transition::Error;
use ssz_rs::prelude::*;

// `Context` for the current state transition
#[derive(Debug, Default)]
pub struct Context<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
    const SYNC_COMMITTEE_SIZE: usize,
> {
    pub spec: Spec,
    pub fork_schedule: ForkSchedule,

    // holds the union of all possible `BeaconState` data
    // phase0 beacon state
    pub genesis_time: u64,
    pub genesis_validators_root: Root,
    pub slot: Slot,
    pub fork: Fork,
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub state_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    pub historical_roots: List<Root, HISTORICAL_ROOTS_LIMIT>,
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: List<Eth1Data, ETH1_DATA_VOTES_BOUND>,
    pub eth1_deposit_index: u64,
    pub validators: List<Validator, VALIDATOR_REGISTRY_LIMIT>,
    pub balances: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
    pub randao_mixes: Vector<Bytes32, EPOCHS_PER_HISTORICAL_VECTOR>,
    pub slashings: Vector<Gwei, EPOCHS_PER_SLASHINGS_VECTOR>,
    pub previous_epoch_attestations:
        List<PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>, PENDING_ATTESTATIONS_BOUND>,
    pub current_epoch_attestations:
        List<PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>, PENDING_ATTESTATIONS_BOUND>,
    pub justification_bits: Bitvector<JUSTIFICATION_BITS_LENGTH>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,

    // altair beacon state
    pub previous_epoch_participation: List<ParticipationFlags, VALIDATOR_REGISTRY_LIMIT>,
    pub current_epoch_participation: List<ParticipationFlags, VALIDATOR_REGISTRY_LIMIT>,
    pub inactivity_scores: List<u64, VALIDATOR_REGISTRY_LIMIT>,
    pub current_sync_committee: SyncCommittee<SYNC_COMMITTEE_SIZE>,
    pub next_sync_committee: SyncCommittee<SYNC_COMMITTEE_SIZE>,
}

impl<
        const SLOTS_PER_HISTORICAL_ROOT: usize,
        const HISTORICAL_ROOTS_LIMIT: usize,
        const ETH1_DATA_VOTES_BOUND: usize,
        const VALIDATOR_REGISTRY_LIMIT: usize,
        const EPOCHS_PER_HISTORICAL_VECTOR: usize,
        const EPOCHS_PER_SLASHINGS_VECTOR: usize,
        const MAX_VALIDATORS_PER_COMMITTEE: usize,
        const PENDING_ATTESTATIONS_BOUND: usize,
        const SYNC_COMMITTEE_SIZE: usize,
    >
    Context<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
        SYNC_COMMITTEE_SIZE,
    >
{
    pub fn new(
        spec: Spec,
        state: BeaconState<
            SLOTS_PER_HISTORICAL_ROOT,
            HISTORICAL_ROOTS_LIMIT,
            ETH1_DATA_VOTES_BOUND,
            VALIDATOR_REGISTRY_LIMIT,
            EPOCHS_PER_HISTORICAL_VECTOR,
            EPOCHS_PER_SLASHINGS_VECTOR,
            MAX_VALIDATORS_PER_COMMITTEE,
            PENDING_ATTESTATIONS_BOUND,
            SYNC_COMMITTEE_SIZE,
        >,
    ) -> Self {
        let fork_schedule = spec.fork_schedule();

        match state {
            BeaconState::Phase0(state) => Self::new_from_phase0(spec, fork_schedule, state),
            BeaconState::Altair(state) => Self::new_from_altair(spec, fork_schedule, state),
        }
    }

    pub fn new_from_phase0(
        spec: Spec,
        fork_schedule: ForkSchedule,
        state: phase0::BeaconState<
            SLOTS_PER_HISTORICAL_ROOT,
            HISTORICAL_ROOTS_LIMIT,
            ETH1_DATA_VOTES_BOUND,
            VALIDATOR_REGISTRY_LIMIT,
            EPOCHS_PER_HISTORICAL_VECTOR,
            EPOCHS_PER_SLASHINGS_VECTOR,
            MAX_VALIDATORS_PER_COMMITTEE,
            PENDING_ATTESTATIONS_BOUND,
        >,
    ) -> Self {
        Self {
            spec,
            fork_schedule,
            genesis_time: state.genesis_time,
            genesis_validators_root: state.genesis_validators_root,
            slot: state.slot,
            fork: state.fork,
            latest_block_header: state.latest_block_header,
            block_roots: state.block_roots,
            state_roots: state.state_roots,
            historical_roots: state.historical_roots,
            eth1_data: state.eth1_data,
            eth1_data_votes: state.eth1_data_votes,
            eth1_deposit_index: state.eth1_deposit_index,
            validators: state.validators,
            balances: state.balances,
            randao_mixes: state.randao_mixes,
            slashings: state.slashings,
            previous_epoch_attestations: state.previous_epoch_attestations,
            current_epoch_attestations: state.current_epoch_attestations,
            justification_bits: state.justification_bits,
            previous_justified_checkpoint: state.previous_justified_checkpoint,
            current_justified_checkpoint: state.current_justified_checkpoint,
            finalized_checkpoint: state.finalized_checkpoint,

            ..Default::default()
        }
    }

    pub fn new_from_altair(
        spec: Spec,
        fork_schedule: ForkSchedule,
        state: altair::BeaconState<
            SLOTS_PER_HISTORICAL_ROOT,
            HISTORICAL_ROOTS_LIMIT,
            ETH1_DATA_VOTES_BOUND,
            VALIDATOR_REGISTRY_LIMIT,
            EPOCHS_PER_HISTORICAL_VECTOR,
            EPOCHS_PER_SLASHINGS_VECTOR,
            MAX_VALIDATORS_PER_COMMITTEE,
            SYNC_COMMITTEE_SIZE,
        >,
    ) -> Self {
        Self {
            spec,
            fork_schedule,
            genesis_time: state.genesis_time,
            genesis_validators_root: state.genesis_validators_root,
            slot: state.slot,
            fork: state.fork,
            latest_block_header: state.latest_block_header,
            block_roots: state.block_roots,
            state_roots: state.state_roots,
            historical_roots: state.historical_roots,
            eth1_data: state.eth1_data,
            eth1_data_votes: state.eth1_data_votes,
            eth1_deposit_index: state.eth1_deposit_index,
            validators: state.validators,
            balances: state.balances,
            randao_mixes: state.randao_mixes,
            slashings: state.slashings,
            justification_bits: state.justification_bits,
            previous_justified_checkpoint: state.previous_justified_checkpoint,
            current_justified_checkpoint: state.current_justified_checkpoint,
            finalized_checkpoint: state.finalized_checkpoint,
            previous_epoch_participation: state.previous_epoch_attestations,
            current_epoch_participation: state.current_epoch_attestations,
            inactivity_scores: state.inactivity_scores,
            current_sync_committee: state.current_sync_committee,
            next_sync_committee: state.next_sync_committee,

            ..Default::default()
        }
    }

    pub fn for_mainnet(
        state: BeaconState<
            SLOTS_PER_HISTORICAL_ROOT,
            HISTORICAL_ROOTS_LIMIT,
            ETH1_DATA_VOTES_BOUND,
            VALIDATOR_REGISTRY_LIMIT,
            EPOCHS_PER_HISTORICAL_VECTOR,
            EPOCHS_PER_SLASHINGS_VECTOR,
            MAX_VALIDATORS_PER_COMMITTEE,
            PENDING_ATTESTATIONS_BOUND,
            SYNC_COMMITTEE_SIZE,
        >,
    ) -> Self {
        Self::new(Spec::for_mainnet(), state)
    }

    pub fn for_minimal(
        state: BeaconState<
            SLOTS_PER_HISTORICAL_ROOT,
            HISTORICAL_ROOTS_LIMIT,
            ETH1_DATA_VOTES_BOUND,
            VALIDATOR_REGISTRY_LIMIT,
            EPOCHS_PER_HISTORICAL_VECTOR,
            EPOCHS_PER_SLASHINGS_VECTOR,
            MAX_VALIDATORS_PER_COMMITTEE,
            PENDING_ATTESTATIONS_BOUND,
            SYNC_COMMITTEE_SIZE,
        >,
    ) -> Self {
        unimplemented!()
    }

    // Return the state root for the current state in `self`
    pub fn state_root(&mut self) -> Result<Node, Error> {
        // TODO compute state root
        Ok(Node::default())
    }
}
