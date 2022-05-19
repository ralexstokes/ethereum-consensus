mod block_processing;
mod context;
// TODO make mod private once the helpers here have been integated
pub mod epoch_processing;
pub mod genesis;
mod slot_processing;

use crate::crypto::{fast_aggregate_verify, hash, Signature as BLSSignature};
use crate::domains::{DomainType, SigningData};
use crate::phase0::beacon_block::{BeaconBlockHeader, SignedBeaconBlock};
use crate::phase0::beacon_state::BeaconState;
use crate::phase0::fork::ForkData;
use crate::phase0::operations::{Attestation, AttestationData, Checkpoint, IndexedAttestation};
use crate::phase0::state_transition::block_processing::process_block;
use crate::phase0::state_transition::slot_processing::process_slots;
use crate::phase0::validator::Validator;
use crate::primitives::{
    Bytes32, CommitteeIndex, Domain, Epoch, ForkDigest, Gwei, Root, Slot, ValidatorIndex, Version,
    FAR_FUTURE_EPOCH, GENESIS_EPOCH,
};
pub use context::Context;
use ssz_rs::prelude::*;
use std::cmp;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Merkleization(#[from] MerkleizationError),
    #[error("{0}")]
    SimpleSerialize(#[from] SimpleSerializeError),
    #[error("requested element {requested} but collection only has {bound} elements")]
    OutOfBounds { requested: usize, bound: usize },
    #[error("invalid signature")]
    InvalidSignature,
    #[error("collection cannot be empty")]
    CollectionCannotBeEmpty,
    #[error("given index {index} is greater than the total amount of indices {total}")]
    InvalidShufflingIndex { index: usize, total: usize },
    #[error("slot {requested} is outside of allowed range ({lower_bound}, {upper_bound})")]
    SlotOutOfRange {
        requested: Slot,
        lower_bound: Slot,
        upper_bound: Slot,
    },
    #[error("overflow")]
    Overflow,
    #[error("{0}")]
    InvalidBlock(InvalidBlock),
    #[error("an invalid transition to a past slot {requested} from slot {current}")]
    TransitionToPreviousSlot { current: Slot, requested: Slot },
    #[error("invalid state root")]
    InvalidStateRoot,
    #[error(
    "the requested epoch {requested} is not in the required current epoch {current} or previous epoch {previous}"
    )]
    InvalidEpoch {
        requested: Epoch,
        previous: Epoch,
        current: Epoch,
    },
}

#[derive(Debug, Error)]
pub enum InvalidBlock {
    #[error("invalid beacon block header: {0}")]
    Header(InvalidBeaconBlockHeader),
    #[error("invalid operation: {0}")]
    InvalidOperation(InvalidOperation),
}

#[derive(Debug, Error)]
pub enum InvalidOperation {
    #[error("invalid attestation: {0}")]
    Attestation(InvalidAttestation),
    #[error("invalid indexed attestation: {0}")]
    IndexedAttestation(InvalidIndexedAttestation),
    #[error("invalid deposit: {0}")]
    Deposit(InvalidDeposit),
    #[error("invalid randao (BLS signature): {0:?}")]
    Randao(BLSSignature),
    #[error("invalid proposer slashing: {0}")]
    ProposerSlashing(InvalidProposerSlashing),
    #[error("invalid attester slashing: {0}")]
    AttesterSlashing(InvalidAttesterSlashing),
    #[error("invalid voluntary exit: {0}")]
    VoluntaryExit(InvalidVoluntaryExit),
}

#[derive(Debug, Error)]
pub enum InvalidBeaconBlockHeader {
    #[error("mismatch between state slot {state_slot} and block slot {block_slot}")]
    StateSlotMismatch { state_slot: Slot, block_slot: Slot },
    #[error("mismatch between the block's parent root {expected:?} and the expected parent root {provided:?}")]
    ParentBlockRootMismatch { expected: Root, provided: Root },
    #[error("proposer with index {0} is slashed")]
    ProposerSlashed(ValidatorIndex),
    #[error("block slot {block_slot} is older than the latest block header slot {latest_block_header_slot}")]
    OlderThanLatestBlockHeader {
        block_slot: Slot,
        latest_block_header_slot: Slot,
    },
    #[error("mismatch between the block proposer index {block_proposer_index} and the state proposer index {proposer_index}")]
    ProposerIndexMismatch {
        block_proposer_index: ValidatorIndex,
        proposer_index: ValidatorIndex,
    },
}

#[derive(Debug, Error)]
pub enum InvalidAttestation {
    #[error("expected length of {expected_length} in bitfield but had length {length}")]
    Bitfield {
        expected_length: usize,
        length: usize,
    },
    #[error("invalid target epoch {target}, not current ({current}) or previous epochs")]
    InvalidTargetEpoch { target: Epoch, current: Epoch },
    #[error("invalid slot {slot} (in epoch {epoch}) based on target epoch {target}")]
    InvalidSlot {
        slot: Slot,
        epoch: Epoch,
        target: Epoch,
    },
    #[error("attestation at slot {attestation_slot} is not timely for state slot {state_slot}, outside of range [{lower_bound}, {upper_bound}]")]
    NotTimely {
        state_slot: Slot,
        attestation_slot: Slot,
        lower_bound: Slot,
        upper_bound: Slot,
    },
    #[error("attestation's index {index} exceeds the current committee count {upper_bound}")]
    InvalidIndex { index: usize, upper_bound: usize },
    #[error("attestation's source checkpoint {source_checkpoint:?} does not match the expected checkpoint {expected:?} (in epoch {current})")]
    InvalidSource {
        expected: Checkpoint,
        source_checkpoint: Checkpoint,
        current: Epoch,
    },
}

#[derive(Debug, Error)]
pub enum InvalidIndexedAttestation {
    #[error("attesting indices are empty")]
    AttestingIndicesEmpty,
    #[error("attesting indices are duplicated")]
    DuplicateIndices(Vec<ValidatorIndex>),
    #[error("attesting indices are not sorted")]
    AttestingIndicesNotSorted,
}

#[derive(Debug, Error)]
pub enum InvalidDeposit {
    #[error("expected {expected} deposits but only had {count} deposits")]
    IncorrectCount { expected: usize, count: usize },
    #[error("merkle validation failed for tree with depth {depth} and root {root:?} at index {index} for leaf {leaf:?} and branch {branch:?}")]
    InvalidProof {
        leaf: Node,
        branch: Vec<Node>,
        depth: usize,
        index: usize,
        root: Root,
    },
    #[error("invalid signature for deposit: {0:?}")]
    InvalidSignature(BLSSignature),
}

#[derive(Debug, Error)]
pub enum InvalidProposerSlashing {
    #[error("different slots: {0} vs. {1}")]
    SlotMismatch(Slot, Slot),
    #[error("different proposers: {0} vs. {1}")]
    ProposerMismatch(ValidatorIndex, ValidatorIndex),
    #[error("headers are equal: {0:?}")]
    HeadersAreEqual(BeaconBlockHeader),
    #[error("proposer with index {0} is not slashable")]
    ProposerIsNotSlashable(ValidatorIndex),
    #[error("header has invalid signature: {0:?}")]
    InvalidSignature(BLSSignature),
}

#[derive(Debug, Error)]
pub enum InvalidAttesterSlashing {
    #[error("attestation data is not slashable: {0:?} vs. {1:?}")]
    NotSlashable(Box<AttestationData>, Box<AttestationData>),
    #[error("no validator was slashed across indices: {0:?}")]
    NoSlashings(Vec<ValidatorIndex>),
}

#[derive(Debug, Error)]
pub enum InvalidVoluntaryExit {
    #[error("validator is not active in the current epoch {0}")]
    InactiveValidator(Epoch),
    #[error("validator {index} already exited in {epoch}")]
    ValidatorAlreadyExited { index: ValidatorIndex, epoch: Epoch },
    #[error("exit in epoch {exit_epoch} is not eligible for processing in current epoch {current_epoch}")]
    EarlyExit {
        current_epoch: Epoch,
        exit_epoch: Epoch,
    },
    #[error("validator needs to be active for a minimum period of time (from epoch {minimum_time_active}, currently in {current_epoch})")]
    ValidatoIsNotActiveForLongEnough {
        current_epoch: Epoch,
        minimum_time_active: Epoch,
    },
    #[error("voluntary exit has invalid signature: {0:?}")]
    InvalidSignature(BLSSignature),
}

pub(crate) fn invalid_header_error(error: InvalidBeaconBlockHeader) -> Error {
    Error::InvalidBlock(InvalidBlock::Header(error))
}

pub(crate) fn invalid_operation_error(error: InvalidOperation) -> Error {
    Error::InvalidBlock(InvalidBlock::InvalidOperation(error))
}

pub fn is_active_validator(validator: &Validator, epoch: Epoch) -> bool {
    validator.activation_epoch <= epoch && epoch < validator.exit_epoch
}

pub fn is_eligible_for_activation_queue(validator: &Validator, context: &Context) -> bool {
    validator.activation_eligibility_epoch == FAR_FUTURE_EPOCH
        && validator.effective_balance == context.max_effective_balance
}

pub fn is_eligible_for_activation<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    validator: &Validator,
) -> bool {
    validator.activation_eligibility_epoch <= state.finalized_checkpoint.epoch
        && validator.activation_epoch == FAR_FUTURE_EPOCH
}

pub fn is_slashable_validator(validator: &Validator, epoch: Epoch) -> bool {
    !validator.slashed
        && validator.activation_epoch <= epoch
        && epoch < validator.withdrawable_epoch
}

pub fn is_slashable_attestation_data(data_1: &AttestationData, data_2: &AttestationData) -> bool {
    let double_vote = data_1 != data_2 && data_1.target.epoch == data_2.target.epoch;
    let surround_vote =
        data_1.source.epoch < data_2.source.epoch && data_2.target.epoch < data_1.target.epoch;
    double_vote || surround_vote
}

pub fn is_valid_indexed_attestation<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    indexed_attestation: &mut IndexedAttestation<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<(), Error> {
    let attesting_indices = &indexed_attestation.attesting_indices;

    if attesting_indices.is_empty() {
        return Err(invalid_operation_error(
            InvalidOperation::IndexedAttestation(InvalidIndexedAttestation::AttestingIndicesEmpty),
        ));
    }

    let is_sorted = attesting_indices
        .windows(2)
        .map(|pair| {
            let a = &pair[0];
            let b = &pair[1];
            a < b
        })
        .all(|x| x);
    if !is_sorted {
        return Err(invalid_operation_error(
            InvalidOperation::IndexedAttestation(
                InvalidIndexedAttestation::AttestingIndicesNotSorted,
            ),
        ));
    }

    let indices: HashSet<usize> = HashSet::from_iter(attesting_indices.iter().cloned());
    if indices.len() != indexed_attestation.attesting_indices.len() {
        let mut seen = HashSet::new();
        let mut duplicates = vec![];
        for i in indices.iter() {
            if seen.contains(i) {
                duplicates.push(*i);
            } else {
                seen.insert(i);
            }
        }
        return Err(invalid_operation_error(
            InvalidOperation::IndexedAttestation(InvalidIndexedAttestation::DuplicateIndices(
                duplicates,
            )),
        ));
    }
    let public_keys = state
        .validators
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if indices.contains(&i) {
                Some(&v.public_key)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let domain = get_domain(
        state,
        DomainType::BeaconAttester,
        Some(indexed_attestation.data.target.epoch),
        context,
    )?;
    let signing_root = compute_signing_root(&mut indexed_attestation.data, domain)?;
    if fast_aggregate_verify(
        &public_keys,
        signing_root.as_bytes(),
        &indexed_attestation.signature,
    ) {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

pub fn verify_block_signature<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
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
        PENDING_ATTESTATIONS_BOUND,
    >,
    signed_block: &mut SignedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    context: &Context,
) -> Result<(), Error> {
    let proposer_index = signed_block.message.proposer_index;
    let proposer = state
        .validators
        .get(proposer_index)
        .ok_or(Error::OutOfBounds {
            requested: proposer_index,
            bound: state.validators.len(),
        })?;
    let domain = get_domain(state, DomainType::BeaconProposer, None, context)?;
    let signing_root = compute_signing_root(&mut signed_block.message, domain)?;

    if proposer
        .public_key
        .verify_signature(signing_root.as_bytes(), &signed_block.signature)
    {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

pub fn get_domain<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    domain_type: DomainType,
    epoch: Option<Epoch>,
    context: &Context,
) -> Result<Domain, Error> {
    let epoch = epoch.unwrap_or_else(|| get_current_epoch(state, context));
    let fork_version = if epoch < state.fork.epoch {
        state.fork.previous_version
    } else {
        state.fork.current_version
    };

    compute_domain(
        domain_type,
        Some(fork_version),
        Some(state.genesis_validators_root),
        context,
    )
}

pub fn get_current_epoch<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    context: &Context,
) -> Epoch {
    compute_epoch_at_slot(state.slot, context)
}

pub fn compute_signing_root<T: SimpleSerialize>(
    ssz_object: &mut T,
    domain: Domain,
) -> Result<Root, Error> {
    let object_root = ssz_object.hash_tree_root()?;

    let mut s = SigningData {
        object_root,
        domain,
    };
    s.hash_tree_root().map_err(Error::Merkleization)
}

pub fn compute_shuffled_index(
    mut index: usize,
    index_count: usize,
    seed: &Bytes32,
    context: &Context,
) -> Result<usize, Error> {
    if index >= index_count {
        return Err(Error::InvalidShufflingIndex {
            index,
            total: index_count,
        });
    }

    let mut pivot_input = [0u8; 33];
    pivot_input[..32].copy_from_slice(seed.as_ref());
    let mut source_input = [0u8; 37];
    source_input[..32].copy_from_slice(seed.as_ref());
    for current_round in 0..context.shuffle_round_count {
        pivot_input[32] = current_round as u8;
        let pivot_bytes: [u8; 8] = hash(pivot_input).as_ref()[..8].try_into().unwrap();

        let pivot = (u64::from_le_bytes(pivot_bytes) as usize) % index_count;
        let flip = (pivot + index_count - index) % index_count;
        let position = cmp::max(index, flip);
        let position_bytes: [u8; 4] = ((position / 256) as u32).to_le_bytes();

        source_input[32] = current_round as u8;
        source_input[33..].copy_from_slice(&position_bytes);

        let source = hash(source_input);

        let byte = source.as_ref()[(position % 256) / 8];
        let bit = (byte >> (position % 8)) % 2;
        index = if bit != 0 { flip } else { index };
    }

    Ok(index)
}

pub fn compute_proposer_index<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    indices: &[ValidatorIndex],
    seed: &Bytes32,
    context: &Context,
) -> Result<ValidatorIndex, Error> {
    if indices.is_empty() {
        return Err(Error::CollectionCannotBeEmpty);
    }
    let max_byte = u8::MAX as u64;
    let mut i = 0;
    let total = indices.len();

    let mut hash_input = [0u8; 40];
    hash_input[..32].copy_from_slice(seed.as_ref());
    loop {
        let shuffled_index = compute_shuffled_index((i % total) as usize, total, seed, context)?;
        let candidate_index = indices[shuffled_index];

        let i_bytes: [u8; 8] = (i / 32).to_le_bytes();
        hash_input[32..].copy_from_slice(&i_bytes);
        let random_byte = hash(hash_input).as_ref()[(i % 32)] as u64;

        let effective_balance = state.validators[candidate_index].effective_balance;
        if effective_balance * max_byte >= context.max_effective_balance * random_byte {
            return Ok(candidate_index);
        }
        i += 1
    }
}

pub fn compute_committee(
    indices: &[ValidatorIndex],
    seed: &Bytes32,
    index: usize,
    count: usize,
    context: &Context,
) -> Result<Vec<ValidatorIndex>, Error> {
    let mut committee = vec![0usize; count];
    let start = (indices.len() * index) / count;
    let end = (indices.len()) * (index + 1) / count;
    for i in start..end {
        let index = compute_shuffled_index(i, indices.len(), seed, context)?;
        committee[index] = indices[index];
    }
    Ok(committee)
}

pub fn compute_epoch_at_slot(slot: Slot, context: &Context) -> Epoch {
    slot / context.slots_per_epoch
}

pub fn compute_start_slot_at_epoch(epoch: Epoch, context: &Context) -> Slot {
    epoch * context.slots_per_epoch
}

pub fn compute_activation_exit_epoch(epoch: Epoch, context: &Context) -> Epoch {
    epoch + 1 + context.max_seed_lookahead
}

pub fn compute_fork_digest(
    current_version: Version,
    genesis_validators_root: Root,
) -> Result<ForkDigest, Error> {
    let fork_data_root = compute_fork_data_root(current_version, genesis_validators_root)?;
    let digest = &fork_data_root.as_ref()[..4];
    Ok(digest.try_into().expect("should not fail"))
}

pub fn compute_domain(
    domain_type: DomainType,
    fork_version: Option<Version>,
    genesis_validators_root: Option<Root>,
    context: &Context,
) -> Result<Domain, Error> {
    let fork_version = fork_version.unwrap_or(context.genesis_fork_version);
    let genesis_validators_root = genesis_validators_root.unwrap_or_default();
    let fork_data_root = compute_fork_data_root(fork_version, genesis_validators_root)?;

    let mut domain = Domain::default();
    domain[..4].copy_from_slice(&domain_type.as_bytes());
    domain[4..].copy_from_slice(&fork_data_root.as_ref()[..28]);
    Ok(domain)
}

pub fn compute_fork_data_root(
    current_version: Version,
    genesis_validators_root: Root,
) -> Result<Root, Error> {
    ForkData {
        current_version,
        genesis_validators_root,
    }
    .hash_tree_root()
    .map_err(Error::Merkleization)
}

pub fn get_previous_epoch<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    context: &Context,
) -> Epoch {
    let current_epoch = get_current_epoch(state, context);
    if current_epoch == GENESIS_EPOCH {
        GENESIS_EPOCH
    } else {
        current_epoch - 1
    }
}

pub fn get_block_root<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
    context: &Context,
) -> Result<&'a Root, Error> {
    get_block_root_at_slot(state, compute_start_slot_at_epoch(epoch, context))
}

pub fn get_block_root_at_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    slot: Slot,
) -> Result<&Root, Error> {
    if slot < state.slot || state.slot <= (slot + SLOTS_PER_HISTORICAL_ROOT as Slot) {
        return Err(Error::SlotOutOfRange {
            requested: slot,
            lower_bound: state.slot - 1,
            upper_bound: state.slot + SLOTS_PER_HISTORICAL_ROOT as Slot,
        });
    }
    Ok(&state.block_roots[(slot as usize % SLOTS_PER_HISTORICAL_ROOT)])
}

pub fn get_randao_mix<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
) -> &Bytes32 {
    let epoch = epoch as usize % EPOCHS_PER_HISTORICAL_VECTOR;
    &state.randao_mixes[epoch]
}

pub fn get_active_validator_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
) -> Vec<ValidatorIndex> {
    let mut active = Vec::with_capacity(state.validators.len());

    for (i, v) in state.validators.iter().enumerate() {
        if is_active_validator(v, epoch) {
            active.push(i)
        }
    }
    active
}

pub fn get_validator_churn_limit<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    context: &Context,
) -> usize {
    let active_validator_indices =
        get_active_validator_indices(state, get_current_epoch(state, context));
    u64::max(
        context.min_per_epoch_churn_limit,
        active_validator_indices.len() as u64 / context.churn_limit_quotient,
    ) as usize
}

pub fn get_seed<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
    domain_type: DomainType,
    context: &Context,
) -> Bytes32 {
    let mix_epoch =
        epoch + (context.epochs_per_historical_vector as u64 - context.min_seed_lookahead) - 1;
    let mix = get_randao_mix(state, mix_epoch);
    let mut input = [0u8; 44];
    input[..4].copy_from_slice(&domain_type.as_bytes());
    input[4..12].copy_from_slice(&epoch.to_le_bytes());
    input[12..].copy_from_slice(mix.as_ref());
    hash(input)
}

pub fn get_committee_count_per_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
    context: &Context,
) -> usize {
    u64::max(
        1,
        u64::min(
            context.max_committees_per_slot,
            get_active_validator_indices(state, epoch).len() as u64
                / context.slots_per_epoch
                / context.target_committee_size,
        ),
    ) as usize
}

pub fn get_beacon_committee<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    slot: Slot,
    index: CommitteeIndex,
    context: &Context,
) -> Result<Vec<ValidatorIndex>, Error> {
    let epoch = compute_epoch_at_slot(slot, context);
    let committees_per_slot = get_committee_count_per_slot(state, epoch, context);
    let indices = get_active_validator_indices(state, epoch);
    let seed = get_seed(state, epoch, DomainType::BeaconAttester, context);
    let index = (slot % context.slots_per_epoch) * committees_per_slot as u64 + index as u64;
    let count = committees_per_slot as u64 * context.slots_per_epoch;
    compute_committee(&indices, &seed, index as usize, count as usize, context)
}

pub fn get_beacon_proposer_index<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    context: &Context,
) -> Result<ValidatorIndex, Error> {
    let epoch = get_current_epoch(state, context);
    let mut input = [0u8; 40];
    input[..32]
        .copy_from_slice(get_seed(state, epoch, DomainType::BeaconProposer, context).as_ref());
    input[32..40].copy_from_slice(&state.slot.to_le_bytes());
    let seed = hash(input);
    let indices = get_active_validator_indices(state, epoch);
    compute_proposer_index(state, &indices, &seed, context)
}

pub fn get_total_balance<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    indices: &HashSet<ValidatorIndex>,
    context: &Context,
) -> Result<Gwei, Error> {
    let total_balance = indices
        .iter()
        .try_fold(Gwei::default(), |acc, i| {
            acc.checked_add(state.validators[*i].effective_balance)
        })
        .ok_or(Error::Overflow)?;
    Ok(u64::max(total_balance, context.effective_balance_increment))
}

pub fn get_total_active_balance<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    context: &Context,
) -> Result<Gwei, Error> {
    let indices = get_active_validator_indices(state, get_current_epoch(state, context));
    get_total_balance(state, &HashSet::from_iter(indices), context)
}

pub fn get_indexed_attestation<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    attestation: &Attestation<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<IndexedAttestation<MAX_VALIDATORS_PER_COMMITTEE>, Error> {
    let bits = &attestation.aggregation_bits;
    let mut attesting_indices = get_attesting_indices(state, &attestation.data, bits, context)?
        .into_iter()
        .collect::<Vec<_>>();
    // NOTE: the spec uses a "stable" sort; however, we cannot discriminate
    // a stable from unstable sort on the primitive `ValidatorIndex` type here.
    attesting_indices.sort_unstable();

    let attesting_indices = attesting_indices.try_into()?;

    Ok(IndexedAttestation {
        attesting_indices,
        data: attestation.data.clone(),
        signature: attestation.signature.clone(),
    })
}

pub fn get_attesting_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    data: &AttestationData,
    bits: &Bitlist<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<HashSet<ValidatorIndex>, Error> {
    let committee = get_beacon_committee(state, data.slot, data.index, context)?;

    if bits.len() != committee.len() {
        return Err(invalid_operation_error(InvalidOperation::Attestation(
            InvalidAttestation::Bitfield {
                expected_length: committee.len(),
                length: bits.len(),
            },
        )));
    }

    let mut indices = HashSet::with_capacity(bits.capacity());

    for (i, validator_index) in committee.iter().enumerate() {
        if bits[i] {
            indices.insert(*validator_index);
        }
    }

    Ok(indices)
}

pub fn increase_balance<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    index: ValidatorIndex,
    delta: Gwei,
) {
    state.balances[index] += delta
}

pub fn decrease_balance<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    index: ValidatorIndex,
    delta: Gwei,
) {
    if delta > state.balances[index] {
        state.balances[index] = 0
    } else {
        state.balances[index] -= delta
    }
}

pub fn initiate_validator_exit<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    index: ValidatorIndex,
    context: &Context,
) {
    if state.validators[index].exit_epoch != FAR_FUTURE_EPOCH {
        return;
    }

    let mut exit_epochs: Vec<Epoch> = state
        .validators
        .iter()
        .filter(|v| v.exit_epoch != FAR_FUTURE_EPOCH)
        .map(|v| v.exit_epoch)
        .collect();

    exit_epochs.push(compute_activation_exit_epoch(
        get_current_epoch(state, context),
        context,
    ));

    let mut exit_queue_epoch = *exit_epochs.iter().max().unwrap();

    let exit_queue_churn = state
        .validators
        .iter()
        .filter(|v| v.exit_epoch == exit_queue_epoch)
        .count();

    if exit_queue_churn >= get_validator_churn_limit(state, context) as usize {
        exit_queue_epoch += 1
    }

    state.validators[index].exit_epoch = exit_queue_epoch;
    state.validators[index].withdrawable_epoch =
        state.validators[index].exit_epoch + context.min_validator_withdrawability_delay;
}

pub fn slash_validator<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    slashed_index: ValidatorIndex,
    whistleblower_index: Option<ValidatorIndex>,
    context: &Context,
) -> Result<(), Error> {
    let epoch = get_current_epoch(state, context);
    initiate_validator_exit(state, slashed_index, context);
    state.validators[slashed_index].slashed = true;
    state.validators[slashed_index].withdrawable_epoch = u64::max(
        state.validators[slashed_index].withdrawable_epoch,
        epoch + context.epochs_per_slashings_vector as u64,
    );
    let slashings_index = epoch as usize % EPOCHS_PER_SLASHINGS_VECTOR;
    state.slashings[slashings_index] += state.validators[slashed_index].effective_balance;
    decrease_balance(
        state,
        slashed_index,
        state.validators[slashed_index].effective_balance / context.min_slashing_penalty_quotient,
    );

    let proposer_index = get_beacon_proposer_index(state, context)?;

    let whistleblower_index = whistleblower_index.unwrap_or(proposer_index);

    let whistleblower_reward =
        state.validators[slashed_index].effective_balance / context.whistleblower_reward_quotient;
    let proposer_reward = whistleblower_reward / context.proposer_reward_quotient;
    increase_balance(state, proposer_index, proposer_reward);
    increase_balance(
        state,
        whistleblower_index,
        whistleblower_reward - proposer_reward,
    );
    Ok(())
}

pub fn state_transition<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    signed_block: &mut SignedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    validate_result: Option<bool>,
    context: &Context,
) -> Result<(), Error> {
    let validate_result = validate_result.unwrap_or(true);
    let slot = signed_block.message.slot;

    process_slots(state, slot, context)?;
    if validate_result {
        verify_block_signature(state, signed_block, context)?;
    }
    let block = &mut signed_block.message;
    process_block(state, block, context)?;
    if validate_result && block.state_root != state.hash_tree_root()? {
        return Err(Error::InvalidStateRoot);
    }

    Ok(())
}
