mod block_processing;
mod epoch_processing;

use crate::crypto::{fast_aggregate_verify, hash};
use crate::domains::{DomainType, SigningData};
use crate::phase0::beacon_block::SignedBeaconBlock;
use crate::phase0::beacon_state::BeaconState;
use crate::phase0::fork::ForkData;
use crate::phase0::operations::{AttestationData, IndexedAttestation};
use crate::phase0::validator::Validator;
use crate::primitives::{
    Bytes32, Domain, Epoch, ForkDigest, Gwei, Root, Slot, ValidatorIndex, Version, FAR_FUTURE_EPOCH,
};
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
}

pub fn is_active_validator(validator: Validator, epoch: Epoch) -> bool {
    validator.activation_epoch <= epoch && epoch < validator.exit_epoch
}

pub fn is_eligible_for_activation_queue<const MAX_EFFECTIVE_BALANCE: Gwei>(
    validator: Validator,
) -> bool {
    validator.activation_eligibility_epoch == FAR_FUTURE_EPOCH
        && validator.effective_balance == MAX_EFFECTIVE_BALANCE
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
    const SLOTS_PER_EPOCH: Slot,
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

    let domain = get_domain::<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
        SLOTS_PER_EPOCH,
    >(
        &state,
        DomainType::BeaconAttester,
        Some(indexed_attestation.data.target.epoch),
    )?;
    let signing_root = compute_signing_root(&indexed_attestation.data, domain)?;
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
    const SLOTS_PER_EPOCH: Slot,
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
) -> bool {
    let proposer_index = signed_block.message.proposer_index;
    let proposer = state
        .validators
        .get(proposer_index)
        .expect("failed to get validator");
    let domain = match get_domain::<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
        SLOTS_PER_EPOCH,
    >(&state, DomainType::BeaconProposer, None)
    {
        Ok(domain) => domain,
        Err(_) => return false,
    };
    let signing_root = match compute_signing_root(&signed_block.message, domain) {
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
    const SLOTS_PER_EPOCH: Slot,
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
) -> Result<Domain, Error> {
    let epoch = epoch.unwrap_or_else(|| {
        get_current_epoch::<
            SLOTS_PER_HISTORICAL_ROOT,
            HISTORICAL_ROOTS_LIMIT,
            ETH1_DATA_VOTES_BOUND,
            VALIDATOR_REGISTRY_LIMIT,
            EPOCHS_PER_HISTORICAL_VECTOR,
            EPOCHS_PER_SLASHINGS_VECTOR,
            MAX_VALIDATORS_PER_COMMITTEE,
            PENDING_ATTESTATIONS_BOUND,
            SLOTS_PER_EPOCH,
        >(state)
    });
    let fork_version = if epoch < state.fork.epoch {
        state.fork.previous_version
    } else {
        state.fork.current_version
    };

    compute_domain(
        domain_type,
        Some(fork_version),
        Some(state.genesis_validators_root),
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
    const SLOTS_PER_EPOCH: Slot,
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
) -> Epoch {
    compute_epoch_at_slot::<SLOTS_PER_EPOCH>(state.slot)
}

pub fn compute_signing_root<T: SimpleSerialize>(
    ssz_object: &T,
    domain: Domain,
) -> Result<Root, Error> {
    let context = MerkleizationContext::new();
    let object_root = ssz_object.hash_tree_root(&context)?;

    let s = SigningData {
        object_root,
        domain,
    };
    s.hash_tree_root(&context).map_err(Error::Merkleization)
}

pub fn bytes_to_int64(slice: &[u8]) -> u64 {
    let mut bytes = [0; 8];
    bytes.copy_from_slice(&slice[0..8]);
    u64::from_le_bytes(bytes)
}

pub fn compute_shuffled_index<const SHUFFLE_ROUND_COUNT: usize>(
    index: usize,
    index_count: usize,
    seed: &Bytes32,
) -> Option<usize> {
    if index < index_count {
        return None;
    }

    let mut index = index;

    for current_round in 0..SHUFFLE_ROUND_COUNT {
        let round_bytes: [u8; 1] = (current_round as u8).to_le_bytes();

        let mut h: Vector<u8, 33> = Vector::default();
        h[0..32].copy_from_slice(seed.as_ref());
        h[32..33].copy_from_slice(&round_bytes);

        let mut v: [u8; 8] = [0u8; 8];
        v.copy_from_slice(hash(h.as_slice()).as_ref());

        let pivot = (bytes_to_int64(&v[..]) as usize) % index_count;

        let flip = (pivot + index_count - index) % index_count;
        let position = cmp::max(index, flip);
        let position_bytes: [u8; 4] = ((position / 256) as u32).to_le_bytes();

        let mut h: Vector<u8, 37> = Vector::default();
        h[0..32].copy_from_slice(seed.as_ref());
        h[32..33].copy_from_slice(&round_bytes);
        h[33..37].copy_from_slice(&position_bytes);

        let byte = (hash(h.as_slice()).as_ref()[(position % 256) / 8]) as u8;
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
    const MAX_EFFECTIVE_BALANCE: Gwei,
    const SHUFFLE_ROUND_COUNT: usize,
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
) -> Result<ValidatorIndex, Error> {
    if indices.is_empty() {
        return Err(Error::InsufficientValidators);
    }
    let max_byte = u8::MAX as u64;
    let mut i: usize = 0;
    let total: usize = indices.len();

    loop {
        let shuffled_index =
            compute_shuffled_index::<SHUFFLE_ROUND_COUNT>((i % total) as usize, total, seed)
                .ok_or(Error::Shuffle)?;
        let candidate_index = indices[shuffled_index];

        let mut h: Vector<u8, 40> = Vector::default();
        let i_bytes: [u8; 8] = (i / 32).to_le_bytes();
        h[0..32].copy_from_slice(seed.as_ref());
        h[32..33].copy_from_slice(&i_bytes);
        let random_byte = hash(h.as_slice()).as_ref()[(i % 32)] as u64;

        let effective_balance = state.validators[candidate_index].effective_balance;
        if effective_balance * max_byte >= MAX_EFFECTIVE_BALANCE * random_byte {
            return Ok(candidate_index);
        }
        i += 1
    }
}

pub fn compute_committee<const SHUFFLE_ROUND_COUNT: usize>(
    indices: &[ValidatorIndex],
    seed: Bytes32,
    index: usize,
    count: usize,
) -> Result<&[ValidatorIndex], Error> {
    let committee: &mut [ValidatorIndex] = &mut [];
    let start = (indices.len() * index) / count;
    let end = (indices.len()) * (index + 1) / count;
    for i in start..end {
        let index = compute_shuffled_index::<SHUFFLE_ROUND_COUNT>(i, indices.len(), &seed)
            .ok_or(Error::Shuffle)?;
        committee[index] = indices[index];
    }
    Ok(committee)
}

pub fn compute_epoch_at_slot<const SLOTS_PER_EPOCH: Slot>(slot: Slot) -> Epoch {
    slot / SLOTS_PER_EPOCH
}

pub fn compute_start_slot_at_epoch<const SLOTS_PER_EPOCH: Slot>(epoch: Epoch) -> Slot {
    epoch * SLOTS_PER_EPOCH
}

pub fn compute_activation_exit_epoch<const MAX_SEED_LOOKAHEAD: Epoch>(epoch: Epoch) -> Epoch {
    epoch + 1 + MAX_SEED_LOOKAHEAD
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
) -> Result<Domain, Error> {
    // NOTE: `GENESIS_FORK_VERSION` is equivalent to the `Default` impl
    let fork_version = fork_version.unwrap_or_default();
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
    let d = ForkData {
        current_version,
        genesis_validators_root,
    };
    let context = MerkleizationContext::new();
    d.hash_tree_root(&context).map_err(Error::Merkleization)
}
