use crate::crypto::Error as CryptoError;
use crate::phase0::{AttestationData, BeaconBlockHeader, Checkpoint};
use crate::primitives::{BlsSignature, Bytes32, Epoch, Hash32, Root, Slot, ValidatorIndex};
use crate::state_transition::Forks;
use ssz_rs::prelude::*;
use crate::prelude::*;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Merkleization,
    SimpleSerialize,
    Crypto,
    OutOfBounds {
        requested: usize,
        bound: usize,
    },
    CollectionCannotBeEmpty,
    InvalidShufflingIndex {
        index: usize,
        total: usize,
    },
    SlotOutOfRange {
        requested: Slot,
        lower_bound: Slot,
        upper_bound: Slot,
    },
    Overflow,
    Underflow,
    InvalidBlock,
    TransitionToPreviousSlot {
        current: Slot,
        requested: Slot,
    },
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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
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

impl Display for InvalidBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            InvalidBlock::Header => write!(f, "invalid beacon block header"),
            InvalidBlock::InvalidOperation => write!(f, "invalid operation"),
        }
    }
}

#[derive(Debug)]
pub enum InvalidOperation {
    Attestation,
    IndexedAttestation,
    Deposit,
    Randao(BlsSignature),
    ProposerSlashing,
    AttesterSlashing,
    VoluntaryExit,
    SyncAggregate,
    ExecutionPayload,
}

impl Display for InvalidOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidOperation::Attestation => write!(f, "invalid attestation"),
            InvalidOperation::IndexedAttestation => write!(f, "invalid indexed attestation"),
            InvalidOperation::Deposit => write!(f, "invalid deposit"),
            InvalidOperation::Randao(blssignature) => {
                write!(f, "invalid randao (Bls signature): {0:?}", blssignature)
            }
            InvalidOperation::ProposerSlashing => write!(f, "invalid proposer slashing"),
            InvalidOperation::AttesterSlashing => write!(f, "invalid attester slashing"),
            InvalidOperation::VoluntaryExit => write!(f, "invalid voluntary exit"),
            InvalidOperation::SyncAggregate => write!(f, "invalid sync aggregate"),
            InvalidOperation::ExecutionPayload => write!(f, "invalid execution payload"),
        }
    }
}

impl From<InvalidAttestation> for InvalidOperation {
    fn from(_: InvalidAttestation) -> Self {
        InvalidOperation::Attestation
    }
}

impl From<InvalidIndexedAttestation> for InvalidOperation {
    fn from(_: InvalidIndexedAttestation) -> Self {
        InvalidOperation::IndexedAttestation
    }
}

impl From<InvalidDeposit> for InvalidOperation {
    fn from(_: InvalidDeposit) -> Self {
        InvalidOperation::Deposit
    }
}

impl From<InvalidProposerSlashing> for InvalidOperation {
    fn from(_: InvalidProposerSlashing) -> Self {
        InvalidOperation::ProposerSlashing
    }
}

impl From<InvalidAttesterSlashing> for InvalidOperation {
    fn from(_: InvalidAttesterSlashing) -> Self {
        InvalidOperation::AttesterSlashing
    }
}

impl From<InvalidVoluntaryExit> for InvalidOperation {
    fn from(_: InvalidVoluntaryExit) -> Self {
        InvalidOperation::VoluntaryExit
    }
}

impl From<InvalidSyncAggregate> for InvalidOperation {
    fn from(_: InvalidSyncAggregate) -> Self {
        InvalidOperation::SyncAggregate
    }
}

impl From<InvalidExecutionPayload> for InvalidOperation {
    fn from(_: InvalidExecutionPayload) -> Self {
        InvalidOperation::ExecutionPayload
    }
}

#[derive(Debug)]
pub enum InvalidBeaconBlockHeader {
    StateSlotMismatch { state_slot: Slot, block_slot: Slot },
    ParentBlockRootMismatch { expected: Root, provided: Root },
    ProposerSlashed(ValidatorIndex),
    OlderThanLatestBlockHeader {
        block_slot: Slot,
        latest_block_header_slot: Slot,
    },
    ProposerIndexMismatch {
        block_proposer_index: ValidatorIndex,
        proposer_index: ValidatorIndex,
    },
}

impl Display for InvalidBeaconBlockHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidBeaconBlockHeader::StateSlotMismatch{state_slot, block_slot} => write!(f, "mismatch between state slot {} and block slot {}", state_slot, block_slot),
            InvalidBeaconBlockHeader::ParentBlockRootMismatch{expected, provided} => write!(f, "mismatch between the block's parent root {:?} and the expected parent root {:?}", expected, provided),
            InvalidBeaconBlockHeader::ProposerSlashed(validator_index)=> write!(f, "proposer with index {} is slashed", validator_index),
            InvalidBeaconBlockHeader::OlderThanLatestBlockHeader{block_slot, latest_block_header_slot} => write!(f, "block slot {} is older than the latest block header slot {}", block_slot, latest_block_header_slot),
            InvalidBeaconBlockHeader::ProposerIndexMismatch{block_proposer_index, proposer_index} => write!(f, "mismatch between the block proposer index {} and the state proposer index {}", block_proposer_index, proposer_index),
        }
    }
}

#[derive(Debug)]
pub enum InvalidAttestation {
    Bitfield {
        expected_length: usize,
        length: usize,
    },
    InvalidTargetEpoch { target: Epoch, current: Epoch },
    InvalidSlot {
        slot: Slot,
        epoch: Epoch,
        target: Epoch,
    },
    NotTimely {
        state_slot: Slot,
        attestation_slot: Slot,
        lower_bound: Slot,
        upper_bound: Slot,
    },
    InvalidIndex { index: usize, upper_bound: usize },
    InvalidSource {
        expected: Checkpoint,
        source_checkpoint: Checkpoint,
        current: Epoch,
    },
}

impl Display for InvalidAttestation {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidAttestation::Bitfield{expected_length, length} => write!(f, "expected length of {} in bitfield but had length {}", expected_length, length),
            InvalidAttestation::InvalidTargetEpoch{target, current} => write!(f, "invalid target epoch {}, not current ({}) or previous epochs", target, current),
            InvalidAttestation::InvalidSlot{slot, epoch, target} => write!(f, "invalid slot {} (in epoch {}) based on target epoch {}", slot, epoch, target),
            InvalidAttestation::NotTimely{state_slot, attestation_slot, lower_bound, upper_bound} => write!(f, "attestation at slot {} is not timely for state slot {}, outside of range [{}, {}]", attestation_slot, state_slot, lower_bound, upper_bound),
            InvalidAttestation::InvalidIndex{index,  upper_bound} => write!(f, "attestation's index {} exceeds the current committee count {}", index, upper_bound),
            InvalidAttestation::InvalidSource{expected,  source_checkpoint,current} => write!(f, "attestation's source checkpoint {:?} does not match the expected checkpoint {:?} (in epoch {})", source_checkpoint, expected, current),
        }
    }
}

#[derive(Debug)]
pub enum InvalidIndexedAttestation {
    AttestingIndicesEmpty,
    DuplicateIndices(Vec<ValidatorIndex>),
    AttestingIndicesNotSorted,
    InvalidIndex(ValidatorIndex),
}

impl Display for InvalidIndexedAttestation {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidIndexedAttestation::AttestingIndicesEmpty => write!(f, "attesting indices are empty"),
            InvalidIndexedAttestation::DuplicateIndices(validator_indices) => write!(f, "attesting indices are duplicated"),
            InvalidIndexedAttestation::AttestingIndicesNotSorted => write!(f, "attesting indices are not sorted"),
            InvalidIndexedAttestation::InvalidIndex(validator_index) => write!(f, "index in attesting set is invalid for this state"),

        }
    }
}

#[derive(Debug)]
pub enum InvalidDeposit {
    IncorrectCount { expected: usize, count: usize },
    InvalidProof {
        leaf: Node,
        branch: Vec<Node>,
        depth: usize,
        index: usize,
        root: Root,
    },
    InvalidSignature(BlsSignature),
}

impl Display for InvalidDeposit {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidDeposit::IncorrectCount {expected, count} => write!(f, "expected {} deposits but only had {} deposits", expected, count),
            InvalidDeposit::InvalidProof{leaf, branch, depth, index, root} => write!(f, "merkle validation failed for tree with depth {} and root {:?} at index {} for leaf {:?} and branch {:?}", depth, root, index, leaf, branch),
            InvalidIndexedAttestation::InvalidSignature(bls) => write!(f, "invalid signature for deposit: {:?}", bls),
        }
    }
}


#[derive(Debug)]
pub enum InvalidProposerSlashing {
    SlotMismatch(Slot, Slot),
    ProposerMismatch(ValidatorIndex, ValidatorIndex),
    HeadersAreEqual(BeaconBlockHeader),
    ProposerIsNotSlashable(ValidatorIndex),
    InvalidSignature(BlsSignature),
    InvalidIndex(ValidatorIndex),
}

impl Display for InvalidProposerSlashing {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidProposerSlashing::SlotMismatch (slot_1, slot_2) => write!(f, "different slots: {} vs. {}", slot_1, slot_2),
            InvalidProposerSlashing::ProposerMismatch (index_1, index_2) => write!(f, "different proposers: {} vs. {}", index_1, index_2),
            InvalidProposerSlashing::HeadersAreEqual (beacon_block_header) => write!(f, "headers are equal: {:?}", beacon_block_header),
            InvalidProposerSlashing::ProposerIsNotSlashable (index) => write!(f, "proposer with index {} is not slashable", index),
            InvalidProposerSlashing::InvalidSignature (bls) => write!(f, "header has invalid signature: {:?}", bls),
            InvalidProposerSlashing::InvalidIndex (index) => write!(f, "proposer with index {} is not in state", index),
        }
    }
}

#[derive(Debug)]
pub enum InvalidAttesterSlashing {
    NotSlashable(Box<AttestationData>, Box<AttestationData>),
    NoSlashings(Vec<ValidatorIndex>),
}

impl Display for InvalidAttesterSlashing {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidAttesterSlashing::NotSlashable(attestation_data1, attestation_data2) => write!(
                f,
                "attestation data is not slashable {:?}, {:?}",
                attestation_data1, attestation_data2
            ),
            InvalidAttesterSlashing::NoSlashings(indices) => {
                write!(f, "no validator was slashed across indices:, {:?}", indices)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum InvalidVoluntaryExit {
    InvalidIndex(ValidatorIndex),
    InactiveValidator(Epoch),
    ValidatorAlreadyExited { index: ValidatorIndex, epoch: Epoch },
    EarlyExit {
        current_epoch: Epoch,
        exit_epoch: Epoch,
    },
    ValidatorIsNotActiveForLongEnough {
        current_epoch: Epoch,
        minimum_time_active: Epoch,
    },
    InvalidSignature(BlsSignature),
}

impl Display for InvalidVoluntaryExit {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidVoluntaryExit::InvalidIndex (index) => write!(f, "validator with index {} is not in state", index),
            InvalidVoluntaryExit::InactiveValidator (epoch) => write!(f, "validator is not active in the current epoch {}", epoch),
            InvalidVoluntaryExit::ValidatorAlreadyExited {index, epoch} => write!(f, "validator {} already exited in {}", index, epoch),
            InvalidVoluntaryExit::EarlyExit {current_epoch, exit_epoch} => write!(f, "exit in epoch {} is not eligible for processing in current epoch {}", exit_epoch, current_epoch),
            InvalidVoluntaryExit::ValidatorIsNotActiveForLongEnough {current_epoch, minimum_time_active} => write!(f, "validator needs to be active for a minimum period of time (from epoch {}, currently in {})", minimum_time_active, current_epoch),
            InvalidVoluntaryExit::InvalidSignature (blst) => write!(f, "voluntary exit has invalid signature: {:?}", blst),
        }
    }
}

#[derive(Debug)]
pub enum InvalidSyncAggregate {
    InvalidSignature { signature: BlsSignature, root: Node },
}

impl Display for InvalidSyncAggregate {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidSyncAggregate::InvalidSignature {signature, root} => write!(f, "invalid sync committee aggregate signature {} signing over previous slot block root {}", signature, root),
        }
    }
}

#[derive(Debug)]
pub enum InvalidExecutionPayload {
    InvalidParentHash { provided: Hash32, expected: Hash32 },
    InvalidPrevRandao {
        provided: Bytes32,
        expected: Bytes32,
    },
    InvalidTimestamp { provided: u64, expected: u64 },
}

impl Display for InvalidExecutionPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidExecutionPayload::InvalidParentHash {provided, expected} => write!(f, "expected parent hash {} but block has parent hash {}", expected, provided),
            InvalidExecutionPayload::InvalidPrevRandao {provided, expected} => write!(f, "expected randao value {} but block has randao value {}", expected, provided),
            InvalidExecutionPayload::InvalidTimestamp {provided, expected} => write!(f, "expected timestamp {} but block has timestamp {}", expected, provided),
        }
    }
}

pub(crate) fn invalid_header_error(error: InvalidBeaconBlockHeader) -> Error {
    Error::InvalidBlock
}

pub(crate) fn invalid_operation_error(error: InvalidOperation) -> Error {
    Error::InvalidBlock
}
