use core::fmt::{Display, Formatter};
use crate::crypto::Error as CryptoError;
use crate::phase0::{AttestationData, BeaconBlockHeader, Checkpoint};
use crate::primitives::{BlsSignature, Bytes32, Epoch, Hash32, Root, Slot, ValidatorIndex};
use crate::state_transition::Forks;
use ssz_rs::prelude::*;
use thiserror::Error;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Merkleization,
    SimpleSerialize,
    Crypto,
    OutOfBounds { requested: usize, bound: usize },
    CollectionCannotBeEmpty,
    InvalidShufflingIndex { index: usize, total: usize },
    SlotOutOfRange {
        requested: Slot,
        lower_bound: Slot,
        upper_bound: Slot,
    },
    Overflow,
    Underflow,
    InvalidBlock,
    TransitionToPreviousSlot { current: Slot, requested: Slot },
    InvalidStateRoot,

    InvalidEpoch {
        requested: Epoch,
        previous: Epoch,
        current: Epoch,
    },
    IncompatibleForks {
        source_fork: Forks,
        destination_fork: Forks,
    },
}

impl From<MerkleizationError> for Error {
    fn from(_: MerkleizationError) -> Self {
        Error::Merkleization
    }
}

impl From<CryptoError> for Error {
    fn from(_: CryptoError) -> Self {
        Error::Crypto
    }
}

impl From<Box<InvalidBlock>> for Error {
    fn from(_: Box<InvalidBlock>) -> Self {
        Error::InvalidBlock
    }
}

#[derive(Debug)]
pub enum InvalidBlock {
    Header,
    InvalidOperation,
}

impl From<InvalidBeaconBlockHeader> for InvalidBlock {
    fn from(_: InvalidBeaconBlockHeader) -> Self {
        InvalidBlock::Header
    }
}

impl From<InvalidOperation> for InvalidBlock {
    fn from(_: InvalidOperation) -> Self {
        InvalidBlock::InvalidOperation
    }
}

impl From<InvalidOperation> for InvalidBlock {
    fn from(_: InvalidOperation) -> Self {
        InvalidBlock::InvalidOperation
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Error::Merkleization => write!(f, "merkleization error"),
            Error::SimpleSerialize => write!(f, "simple serialize error"),
            Error::Crypto => write!(f, "crypto error"),
            Error::OutOfBounds{requested, bound} => write!(f, "requested element {} but collection only has {} elements", requested, bound),
            Error::CollectionCannotBeEmpty => write!(f, "collection cannot be empty"),
            Error::InvalidShufflingIndex{index, total} => write!(f, "given index {} is greater than the total amount of indices {}", index, total),
            Error::SlotOutOfRange{requested, lower_bound, upper_bound}  => write!(f, "slot {} is outside of allowed range ({}, {})", requested, lower_bound, upper_bound),
            Error::Overflow => write!(f, "overflow error"),
            Error::Underflow => write!(f, "underflow error"),
            Error::InvalidBlock => write!(f, "invalid block"),
            Error::TransitionToPreviousSlot{requested, current} => write!(f, "an invalid transition to a past slot {} from slot {}", requested, current),
            Error::InvalidStateRoot => write!(f, "invalid state root"),
            Error::InvalidEpoch{requested,previous,current} => write!(f, "the requested epoch {} is not in the required current epoch {} or previous epoch {}", requested, current, previous),
            Error::IncompatibleForks{source_fork, destination_fork} => write!(f, "transition requested from a later fork {:?} to an earlier fork {:?}", destination_fork, source_fork),
        }
    }
}


impl Display for InvalidBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            InvalidBlock::Header => write!(f, "invalid beacon block header"),
            InvalidBlock::InvalidOperation => write!(f, "invalid operation"),
        }
    }
}

#[derive(Debug, Error)]
pub enum InvalidOperation {
    #[error("invalid attestation: {0}")]
    Attestation(#[from] InvalidAttestation),
    #[error("invalid indexed attestation: {0}")]
    IndexedAttestation(#[from] InvalidIndexedAttestation),
    #[error("invalid deposit: {0}")]
    Deposit(#[from] InvalidDeposit),
    #[error("invalid randao (Bls signature): {0:?}")]
    Randao(BlsSignature),
    #[error("invalid proposer slashing: {0}")]
    ProposerSlashing(#[from] InvalidProposerSlashing),
    #[error("invalid attester slashing: {0}")]
    AttesterSlashing(#[from] InvalidAttesterSlashing),
    #[error("invalid voluntary exit: {0}")]
    VoluntaryExit(#[from] InvalidVoluntaryExit),
    #[error("invalid sync aggregate: {0}")]
    SyncAggregate(#[from] InvalidSyncAggregate),
    #[error("invalid execution payload: {0}")]
    ExecutionPayload(#[from] InvalidExecutionPayload),
}

#[derive(Debug)]
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
    #[error("index in attesting set is invalid for this state")]
    InvalidIndex(ValidatorIndex),
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
    InvalidSignature(BlsSignature),
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
    InvalidSignature(BlsSignature),
    #[error("proposer with index {0} is not in state")]
    InvalidIndex(ValidatorIndex),
}

#[derive(Debug)]
pub enum InvalidAttesterSlashing {
    NotSlashable(Box<AttestationData>, Box<AttestationData>),
    NoSlashings(Vec<ValidatorIndex>),
}

impl Display for InvalidAttesterSlashing {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            InvalidAttesterSlashing::NotSlashable(attestation_data1, attestation_data2)  => write!(f, "attestation data is not slashable {}, {}", attestation_data1, attestation_data2),
            InvalidAttesterSlashing::NoSlashings(indices)  => write!(f, "no slashings occured for indices, {}", indices),
        }
    }
}

#[derive(Debug, Error)]
pub enum InvalidVoluntaryExit {
    #[error("validator with index {0} is not in state")]
    InvalidIndex(ValidatorIndex),
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
    InvalidSignature(BlsSignature),
}

#[derive(Debug, Error)]
pub enum InvalidSyncAggregate {
    #[error("invalid sync committee aggregate signature {signature} signing over previous slot block root {root}")]
    InvalidSignature { signature: BlsSignature, root: Node },
}

#[derive(Debug, Error)]
pub enum InvalidExecutionPayload {
    #[error("expected parent hash {expected} but block has parent hash {provided}")]
    InvalidParentHash { provided: Hash32, expected: Hash32 },
    #[error("expected randao value {expected} but block has randao value {provided}")]
    InvalidPrevRandao {
        provided: Bytes32,
        expected: Bytes32,
    },
    #[error("expected timestamp {expected} but block has timestamp {provided}")]
    InvalidTimestamp { provided: u64, expected: u64 },
}

pub(crate) fn invalid_header_error(error: InvalidBeaconBlockHeader) -> Error {
    Error::InvalidBlock(Box::new(InvalidBlock::Header(error)))
}

pub(crate) fn invalid_operation_error(error: InvalidOperation) -> Error {
    Error::InvalidBlock(Box::new(InvalidBlock::InvalidOperation(error)))
}
