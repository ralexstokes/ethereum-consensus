mod block_processing;
mod epoch_processing;

use crate::crypto::{fast_aggregate_verify, hash};
use crate::domains::{DomainType, SigningData};
use crate::phase0::beacon_block::SignedBeaconBlock;
use crate::phase0::beacon_state::BeaconState;
use crate::phase0::configs::mainnet::{GENESIS_FORK_VERSION, GENESIS_FORK_VERSION_BYTES};
use crate::phase0::fork::ForkData;
use crate::phase0::mainnet::{MAX_SEED_LOOKAHEAD, SLOTS_PER_EPOCH};
use crate::phase0::operations::{AttestationData, IndexedAttestation};
use crate::phase0::validator::Validator;
use crate::primitives::{
    Bytes32, Domain, Epoch, ForkDigest, Gwei, Root, Slot, Version, FAR_FUTURE_EPOCH,
};
use sha2::digest::generic_array::functional::FunctionalSequence;
use ssz_rs::prelude::*;
use std::collections::HashSet;
use std::io::Bytes;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Block Signature Error")]
    BlockSignatureError,
    #[error("Merkleization Error")]
    MerkleizationError(#[from] MerkleizationError),
    #[error("Deserializen Error")]
    DeserializeError(#[from] DeserializeError),
    #[error("invalid operation")]
    InvalidOperation,
    #[error("invalid signature")]
    InvalidSignature,
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
    let pubkeys = state.validators.iter().enumerate().filter_map(|(i, v)| {
        if indices.contains(&i) {
            Some(&v.pubkey)
        } else {
            None
        }
    });

    let domain = get_domain(
        &state,
        DomainType::BeaconAttester,
        Some(indexed_attestation.data.target.epoch),
    )?;
    let signing_root = compute_signing_root(&indexed_attestation.data, domain)?;
    if fast_aggregate_verify(pubkeys, signing_root, &indexed_attestation.signature) {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

pub fn is_valid_merkle_branch(
    leaf: Bytes32,
    branch: &[Bytes32],
    depth: usize,
    index: usize,
    root: Root,
) -> bool {
    let mut value = leaf;
    for i in 0..depth {
        if (index / 2usize.pow(i as u32)) % 2 != 0 {
            let x = branch[i].xor(value);
            value = hash(x.0.as_slice());
        } else {
            let x = value.xor(branch[i].clone());
            value = hash(x.0.as_slice())
        }
    }
    value.as_bytes() == <ssz_rs::Root as AsRef<[u8]>>::as_ref(&root)
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
    let domain = match get_domain(&state, DomainType::BeaconProposer, None) {
        Ok(domain) => domain,
        Err(_) => return false,
    };
    let signing_root = match compute_signing_root(&signed_block.message, domain) {
        Ok(root) => root,
        Err(_) => return false,
    };

    proposer
        .pubkey
        .verify_signature(signing_root, signed_block.signature)
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
) -> Result<Domain, Error> {
    let epoch = epoch.unwrap_or_else(|| get_current_epoch(&state));
    let fork_version = if epoch < state.fork.epoch {
        Some(state.fork.previous_version.clone())
    } else {
        Some(state.fork.current_version.clone())
    };

    compute_domain(
        domain_type,
        fork_version,
        Some(state.genesis_validators_root.clone()),
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
) -> Epoch {
    compute_epoch_at_slot(state.slot)
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
    s.hash_tree_root(&context)
        .map_err(Error::MerkleizationError)
}

pub fn compute_epoch_at_slot(slot: Slot) -> Epoch {
    slot / SLOTS_PER_EPOCH
}

pub fn compute_start_slot_at_epoch(epoch: Epoch) -> Slot {
    epoch * SLOTS_PER_EPOCH
}

pub fn compute_activation_exit_epoch(epoch: Epoch) -> Epoch {
    epoch + 1 + MAX_SEED_LOOKAHEAD
}

pub fn compute_fork_digest(
    current_version: Version,
    genesis_validators_root: Root,
) -> Result<ForkDigest, Error> {
    let fork_data_root = compute_fork_data_root(current_version, genesis_validators_root)?;
    let fork_digest = fork_data_root[..4]
        .try_into()
        .expect("is the correct length");
    Ok(fork_digest)
}

pub fn compute_domain(
    domain_type: DomainType,
    fork_version: Option<Version>,
    genesis_validators_root: Option<Root>,
) -> Result<Domain, Error> {
    let mut default_fork_version = Vector::<u8, 4>::default();
    default_fork_version.copy_from_slice(&GENESIS_FORK_VERSION[..]);
    let fork_version = fork_version.unwrap_or(default_fork_version);
    let genesis_validators_root = genesis_validators_root.unwrap_or(Root::from_bytes([0u8; 32]));
    let fork_data_root = compute_fork_data_root(fork_version, genesis_validators_root)?;
    let domain_constant = domain_type.as_bytes();
    let fork_data_root_value: &[u8; 32] = fork_data_root.as_ref();

    let mut domain: Domain = Vector::default();
    domain[0..4].copy_from_slice(&domain_constant[..]);
    domain[4..31].copy_from_slice(&fork_data_root_value[..]);
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
    d.hash_tree_root(&context)
        .map_err(Error::MerkleizationError)
}
