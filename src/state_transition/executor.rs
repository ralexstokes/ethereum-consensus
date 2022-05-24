use crate::state_transition::{Context, Error, SignedBeaconBlock};
use ssz_rs::prelude::Node;

pub enum Validation {
    Enabled,
    Disabled,
}

#[derive(Debug)]
pub struct Executor<
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
    context: Context<
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
        const MAX_PROPOSER_SLASHINGS: usize,
        const MAX_ATTESTER_SLASHINGS: usize,
        const MAX_ATTESTATIONS: usize,
        const MAX_DEPOSITS: usize,
        const MAX_VOLUNTARY_EXITS: usize,
    >
    Executor<
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
        context: Context<
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
        Self { context }
    }

    pub fn apply_block(
        &mut self,
        signed_block: &mut SignedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
        >,
    ) -> Result<(), Error> {
        self.apply_block_with_validation(signed_block, Validation::Enabled)
    }

    pub fn apply_block_with_validation(
        &mut self,
        signed_block: &mut SignedBeaconBlock<
            MAX_PROPOSER_SLASHINGS,
            MAX_VALIDATORS_PER_COMMITTEE,
            MAX_ATTESTER_SLASHINGS,
            MAX_ATTESTATIONS,
            MAX_DEPOSITS,
            MAX_VOLUNTARY_EXITS,
            SYNC_COMMITTEE_SIZE,
        >,
        validation: Validation,
    ) -> Result<(), Error> {
        // block = signed_block.message
        // # Process slots (including those with no blocks) since block
        // process_slots(state, block.slot)
        // # Verify signature
        // if validate_result:
        //     assert verify_block_signature(state, signed_block)
        // # Process block
        // process_block(state, block)
        // # Verify state root
        // if validate_result:
        //     assert block.state_root == hash_tree_root(state)
        match signed_block {
            SignedBeaconBlock::Phase0(signed_block) => {
                self.apply_phase0_block_with_validation(signed_block, validation)
            }
            SignedBeaconBlock::Altair(signed_block) => {
                self.apply_altair_block_with_validation(signed_block, validation)
            }
        }

        Ok(())
    }

    pub fn state_root(&mut self) -> Result<Node, Error> {
        self.context.state_root()
    }
}
