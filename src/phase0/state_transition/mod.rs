mod block_processing;
mod context;
mod epoch_processing;

use crate::crypto::{fast_aggregate_verify, hash};
use crate::domains::{DomainType, SigningData};
use crate::phase0::beacon_block::SignedBeaconBlock;
use crate::phase0::beacon_state::BeaconState;
use crate::phase0::fork::ForkData;
use crate::phase0::operations::{AttestationData, IndexedAttestation};
use crate::phase0::validator::Validator;
use crate::primitives::{
    Bytes32, Domain, Epoch, ForkDigest, Root, Slot, ValidatorIndex, Version, FAR_FUTURE_EPOCH,
    GENESIS_EPOCH,
};
pub use context::Context;
use ssz_rs::prelude::*;
use std::cmp;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Block Signature Error")]
    BlockSignature,
    #[error("Fork Digest Error")]
    ForkDigest,
    #[error("Merkleization Error")]
    Merkleization(#[from] MerkleizationError),
    #[error("Deserializen Error")]
    Deserialize(#[from] DeserializeError),
    #[error("invalid operation")]
    InvalidOperation,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("unable to shuffle")]
    Shuffle,
    #[error("insufficient validators")]
    InsufficientValidators,
    #[error("Beacon State Error")]
    BeaconStateError,
}

pub fn is_active_validator(validator: &Validator, epoch: Epoch) -> bool {
    validator.activation_epoch <= epoch && epoch < validator.exit_epoch
}

pub fn is_eligible_for_activation_queue(validator: Validator, context: &Context) -> bool {
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
    state: BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    validator: Validator,
) -> bool {
    validator.activation_eligibility_epoch <= state.finalized_checkpoint.epoch
        && validator.activation_epoch == FAR_FUTURE_EPOCH
}

pub fn is_slashable_validator(validator: Validator, epoch: Epoch) -> bool {
    !validator.slashed
        && validator.activation_epoch <= epoch
        && epoch < validator.withdrawable_epoch
}

pub fn is_slashable_attestation_data(data_1: AttestationData, data_2: AttestationData) -> bool {
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
    state: BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    indexed_attestation: IndexedAttestation<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<(), Error> {
    if indexed_attestation.attesting_indices.is_empty() {
        return Err(Error::InvalidOperation);
    }
    let indices: HashSet<usize> =
        HashSet::from_iter(indexed_attestation.attesting_indices.iter().cloned());
    if indices.len() != indexed_attestation.attesting_indices.len() {
        return Err(Error::InvalidOperation);
    }
    let pubkeys = state
        .validators
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if indices.contains(&i) {
                Some(&v.pubkey)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let domain = get_domain(
        &state,
        DomainType::BeaconAttester,
        Some(indexed_attestation.data.target.epoch),
        context,
    )?;
    let signing_root = compute_signing_root(&indexed_attestation.data, domain, context)?;
    if fast_aggregate_verify(
        &pubkeys,
        signing_root.as_bytes(),
        &indexed_attestation.signature,
    ) {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

pub fn apply_block<
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
    state: BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    _signed_block: SignedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    _context: &Context,
) -> BeaconState<
    SLOTS_PER_HISTORICAL_ROOT,
    HISTORICAL_ROOTS_LIMIT,
    ETH1_DATA_VOTES_BOUND,
    VALIDATOR_REGISTRY_LIMIT,
    EPOCHS_PER_HISTORICAL_VECTOR,
    EPOCHS_PER_SLASHINGS_VECTOR,
    MAX_VALIDATORS_PER_COMMITTEE,
    PENDING_ATTESTATIONS_BOUND,
> {
    state
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
    state: BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    signed_block: SignedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
    >,
    context: &Context,
) -> bool {
    let proposer_index = signed_block.message.proposer_index;
    let proposer = state
        .validators
        .get(proposer_index)
        .expect("failed to get validator");
    let domain = match get_domain(&state, DomainType::BeaconProposer, None, context) {
        Ok(domain) => domain,
        Err(_) => return false,
    };
    let signing_root = match compute_signing_root(&signed_block.message, domain, context) {
        Ok(root) => root,
        Err(_) => return false,
    };

    proposer
        .pubkey
        .verify_signature(signing_root.as_bytes(), &signed_block.signature)
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
    ssz_object: &T,
    domain: Domain,
    context: &Context,
) -> Result<Root, Error> {
    let context = context.for_merkleization();
    let object_root = ssz_object.hash_tree_root(context)?;

    let s = SigningData {
        object_root,
        domain,
    };
    s.hash_tree_root(context).map_err(Error::Merkleization)
}

pub fn compute_shuffled_index(
    index: usize,
    index_count: usize,
    seed: &Bytes32,
    context: &Context,
) -> Option<usize> {
    if index < index_count {
        return None;
    }

    let mut index = index;

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

    Some(index)
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
        return Err(Error::InsufficientValidators);
    }
    let max_byte = u8::MAX as u64;
    let mut i: usize = 0;
    let total: usize = indices.len();

    let mut hash_input = [0u8; 40];
    hash_input[..32].copy_from_slice(seed.as_ref());
    loop {
        let shuffled_index = compute_shuffled_index((i % total) as usize, total, seed, context)
            .ok_or(Error::Shuffle)?;
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

pub fn compute_committee<'a>(
    indices: &'a [ValidatorIndex],
    seed: Bytes32,
    index: usize,
    count: usize,
    context: &Context,
) -> Result<&'a [ValidatorIndex], Error> {
    let committee: &mut [ValidatorIndex] = &mut [];
    let start = (indices.len() * index) / count;
    let end = (indices.len()) * (index + 1) / count;
    for i in start..end {
        let index =
            compute_shuffled_index(i, indices.len(), &seed, context).ok_or(Error::Shuffle)?;
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
    context: &Context,
) -> Result<ForkDigest, Error> {
    let fork_data_root = compute_fork_data_root(current_version, genesis_validators_root, context)?;
    let digest = &fork_data_root.as_ref()[..4];
    Ok(digest.try_into().expect("should not fail"))
}

pub fn compute_domain(
    domain_type: DomainType,
    fork_version: Option<Version>,
    genesis_validators_root: Option<Root>,
    context: &Context,
) -> Result<Domain, Error> {
    // NOTE: `GENESIS_FORK_VERSION` is equivalent to the `Default` impl
    let fork_version = fork_version.unwrap_or_default();
    let genesis_validators_root = genesis_validators_root.unwrap_or_default();
    let fork_data_root = compute_fork_data_root(fork_version, genesis_validators_root, context)?;

    let mut domain = Domain::default();
    domain[..4].copy_from_slice(&domain_type.as_bytes());
    domain[4..].copy_from_slice(&fork_data_root.as_ref()[..28]);
    Ok(domain)
}

pub fn compute_fork_data_root(
    current_version: Version,
    genesis_validators_root: Root,
    context: &Context,
) -> Result<Root, Error> {
    ForkData {
        current_version,
        genesis_validators_root,
    }
    .hash_tree_root(context.for_merkleization())
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
    state: BeaconState<
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
    let current_epoch = get_current_epoch(&state, context);
    if current_epoch == GENESIS_EPOCH {
        GENESIS_EPOCH
    } else {
        current_epoch - 1
    }
}

pub fn get_block_root<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: BeaconState<
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
) -> Result<Root, Error> {
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
    state: BeaconState<
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
) -> Result<Root, Error> {
    if slot < state.slot || state.slot <= (slot + SLOTS_PER_HISTORICAL_ROOT as u64) {
        return Err(Error::BeaconStateError);
    }
    Ok(state.block_roots[(slot as usize % SLOTS_PER_HISTORICAL_ROOT)])
}

pub fn get_randao_mix<
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
) -> &'a Bytes32 {
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
    state: BeaconState<
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
    let mut active: Vec<ValidatorIndex> = Vec::with_capacity(state.validators.len());

    for (i, v) in state.validators.iter().enumerate() {
        if is_active_validator(&v, epoch) {
            active.push(i)
        }
    }
    active
}
