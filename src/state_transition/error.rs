use crate::crypto::Error as CryptoError;
use crate::lib::*;
use crate::phase0::{AttestationData, BeaconBlockHeader, Checkpoint};
use crate::primitives::{BlsSignature, Bytes32, Epoch, Hash32, Root, Slot, ValidatorIndex};
use crate::state_transition::Forks;
use ssz_rs::prelude::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Merkleization(MerkleizationError),
    SimpleSerialize(SimpleSerializeError),
    Crypto(CryptoError),
    #[cfg(all(feature = "serde", feature = "std"))]
    Io(std::io::Error),
    #[cfg(all(feature = "serde", feature = "std"))]
    Yaml(serde_yaml::Error),
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
    InvalidBlock(Box<InvalidBlock>),
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

    UnknownGenesisTime(String),
    #[cfg(feature = "serde")]
    UnknownPreset(String),
}

#[cfg(all(feature = "serde", feature = "std"))]
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

#[cfg(all(feature = "serde", feature = "std"))]
impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::Yaml(error)
    }
}

impl From<SimpleSerializeError> for Error {
    fn from(error: SimpleSerializeError) -> Self {
        Error::SimpleSerialize(error)
    }
}

impl From<MerkleizationError> for Error {
    fn from(error: MerkleizationError) -> Self {
        Error::Merkleization(error)
    }
}

impl From<CryptoError> for Error {
    fn from(error: CryptoError) -> Self {
        Error::Crypto(error)
    }
}

impl From<Box<InvalidBlock>> for Error {
    fn from(error: Box<InvalidBlock>) -> Self {
        Error::InvalidBlock(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Merkleization(_error) => write!(f, "merkleization error"),
            Error::SimpleSerialize(_error) => write!(f, "simple serialize error"),
            #[cfg(all(feature = "serde", feature = "std"))]
            Error::Io(err) => write!(f, "{:?}", err),
            #[cfg(all(feature = "serde", feature = "std"))]
            Error::Yaml(err) => write!(f, "{:?}", err),
            Error::Crypto(_error) => write!(f, "crypto error"),
            Error::OutOfBounds{requested, bound} => write!(f, "requested element {} but collection only has {} elements", requested, bound),
            Error::CollectionCannotBeEmpty => write!(f, "collection cannot be empty"),
            Error::InvalidShufflingIndex{index, total} => write!(f, "given index {} is greater than the total amount of indices {}", index, total),
            Error::SlotOutOfRange{requested, lower_bound, upper_bound}  => write!(f, "slot {} is outside of allowed range ({}, {})", requested, lower_bound, upper_bound),
            Error::Overflow => write!(f, "overflow error"),
            Error::Underflow => write!(f, "underflow error"),
            Error::InvalidBlock(_error) => write!(f, "invalid block"),
            Error::TransitionToPreviousSlot{requested, current} => write!(f, "an invalid transition to a past slot {} from slot {}", requested, current),
            Error::InvalidStateRoot => write!(f, "invalid state root"),
            Error::InvalidEpoch{requested,previous,current} => write!(f, "the requested epoch {} is not in the required current epoch {} or previous epoch {}", requested, current, previous),
            Error::IncompatibleForks{source_fork, destination_fork} => write!(f, "transition requested from a later fork {:?} to an earlier fork {:?}", destination_fork, source_fork),
            Error::UnknownGenesisTime(error) => write!(f, "genesis time unknown for network {}", error),
            #[cfg(feature = "std")]
            Error::UnknownPreset(error) => write!(f, "an unknown preset {} was supplied when constructing context", error),
        }
    }
}

#[derive(Debug)]
pub enum InvalidBlock {
    Header(InvalidBeaconBlockHeader),
    InvalidOperation(InvalidOperation),
}

impl From<InvalidBeaconBlockHeader> for InvalidBlock {
    fn from(error: InvalidBeaconBlockHeader) -> Self {
        InvalidBlock::Header(error)
    }
}

impl From<InvalidOperation> for InvalidBlock {
    fn from(error: InvalidOperation) -> Self {
        InvalidBlock::InvalidOperation(error)
    }
}

impl Display for InvalidBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidBlock::Header(_error) => write!(f, "invalid beacon block header"),
            InvalidBlock::InvalidOperation(_error) => write!(f, "invalid operation"),
        }
    }
}

#[derive(Debug)]
pub enum InvalidOperation {
    Attestation(InvalidAttestation),
    IndexedAttestation(InvalidIndexedAttestation),
    Deposit(InvalidDeposit),
    Randao(BlsSignature),
    ProposerSlashing(InvalidProposerSlashing),
    AttesterSlashing(InvalidAttesterSlashing),
    VoluntaryExit(InvalidVoluntaryExit),
    SyncAggregate(InvalidSyncAggregate),
    ExecutionPayload(InvalidExecutionPayload),
}

impl Display for InvalidOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidOperation::Attestation(_invalid_attestation) => write!(f, "invalid attestation"),
            InvalidOperation::IndexedAttestation(_indexed_attestation) => {
                write!(f, "invalid indexed attestation")
            }
            InvalidOperation::Deposit(_invalid_deposit) => write!(f, "invalid deposit"),
            InvalidOperation::Randao(blssignature) => {
                write!(f, "invalid randao (Bls signature): {0:?}", blssignature)
            }
            InvalidOperation::ProposerSlashing(_invalid_proposer_slashing) => {
                write!(f, "invalid proposer slashing")
            }
            InvalidOperation::AttesterSlashing(_invalifd_attester_slashing) => {
                write!(f, "invalid attester slashing")
            }
            InvalidOperation::VoluntaryExit(_invalid_voluntary_exit) => {
                write!(f, "invalid voluntary exit")
            }
            InvalidOperation::SyncAggregate(_invalid_sync_aggregate) => {
                write!(f, "invalid sync aggregate")
            }
            InvalidOperation::ExecutionPayload(_invalid_execution_payload) => {
                write!(f, "invalid execution payload")
            }
        }
    }
}

impl From<InvalidAttestation> for InvalidOperation {
    fn from(invalid_attestation: InvalidAttestation) -> Self {
        InvalidOperation::Attestation(invalid_attestation)
    }
}

impl From<InvalidIndexedAttestation> for InvalidOperation {
    fn from(invalid: InvalidIndexedAttestation) -> Self {
        InvalidOperation::IndexedAttestation(invalid)
    }
}

impl From<InvalidDeposit> for InvalidOperation {
    fn from(invalid_deposit: InvalidDeposit) -> Self {
        InvalidOperation::Deposit(invalid_deposit)
    }
}

impl From<InvalidProposerSlashing> for InvalidOperation {
    fn from(invalid: InvalidProposerSlashing) -> Self {
        InvalidOperation::ProposerSlashing(invalid)
    }
}

impl From<InvalidAttesterSlashing> for InvalidOperation {
    fn from(invalid: InvalidAttesterSlashing) -> Self {
        InvalidOperation::AttesterSlashing(invalid)
    }
}

impl From<InvalidVoluntaryExit> for InvalidOperation {
    fn from(invalid: InvalidVoluntaryExit) -> Self {
        InvalidOperation::VoluntaryExit(invalid)
    }
}

impl From<InvalidSyncAggregate> for InvalidOperation {
    fn from(sync_aggregate: InvalidSyncAggregate) -> Self {
        InvalidOperation::SyncAggregate(sync_aggregate)
    }
}

impl From<InvalidExecutionPayload> for InvalidOperation {
    fn from(invalid: InvalidExecutionPayload) -> Self {
        InvalidOperation::ExecutionPayload(invalid)
    }
}

#[derive(Debug)]
pub enum InvalidBeaconBlockHeader {
    StateSlotMismatch {
        state_slot: Slot,
        block_slot: Slot,
    },
    ParentBlockRootMismatch {
        expected: Root,
        provided: Root,
    },
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
            InvalidBeaconBlockHeader::StateSlotMismatch {
                state_slot,
                block_slot,
            } => write!(
                f,
                "mismatch between state slot {} and block slot {}",
                state_slot, block_slot
            ),
            InvalidBeaconBlockHeader::ParentBlockRootMismatch { expected, provided } => write!(
                f,
                "mismatch between the block's parent root {:?} and the expected parent root {:?}",
                expected, provided
            ),
            InvalidBeaconBlockHeader::ProposerSlashed(validator_index) => {
                write!(f, "proposer with index {} is slashed", validator_index)
            }
            InvalidBeaconBlockHeader::OlderThanLatestBlockHeader {
                block_slot,
                latest_block_header_slot,
            } => write!(
                f,
                "block slot {} is older than the latest block header slot {}",
                block_slot, latest_block_header_slot
            ),
            InvalidBeaconBlockHeader::ProposerIndexMismatch {
                block_proposer_index,
                proposer_index,
            } => write!(
                f,
                "mismatch between the block proposer index {} and the state proposer index {}",
                block_proposer_index, proposer_index
            ),
        }
    }
}

#[derive(Debug)]
pub enum InvalidAttestation {
    Bitfield {
        expected_length: usize,
        length: usize,
    },
    InvalidTargetEpoch {
        target: Epoch,
        current: Epoch,
    },
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
    InvalidIndex {
        index: usize,
        upper_bound: usize,
    },
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
            InvalidIndexedAttestation::AttestingIndicesEmpty => {
                write!(f, "attesting indices are empty")
            }
            InvalidIndexedAttestation::DuplicateIndices(_validator_indices) => {
                write!(f, "attesting indices are duplicated")
            }
            InvalidIndexedAttestation::AttestingIndicesNotSorted => {
                write!(f, "attesting indices are not sorted")
            }
            InvalidIndexedAttestation::InvalidIndex(_validator_index) => {
                write!(f, "index in attesting set is invalid for this state")
            }
        }
    }
}

#[derive(Debug)]
pub enum InvalidDeposit {
    IncorrectCount {
        expected: usize,
        count: usize,
    },
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
            InvalidDeposit::InvalidSignature(bls) => write!(f, "invalid signature for deposit: {:?}", bls),
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
            InvalidProposerSlashing::SlotMismatch(slot_1, slot_2) => {
                write!(f, "different slots: {} vs. {}", slot_1, slot_2)
            }
            InvalidProposerSlashing::ProposerMismatch(index_1, index_2) => {
                write!(f, "different proposers: {} vs. {}", index_1, index_2)
            }
            InvalidProposerSlashing::HeadersAreEqual(beacon_block_header) => {
                write!(f, "headers are equal: {:?}", beacon_block_header)
            }
            InvalidProposerSlashing::ProposerIsNotSlashable(index) => {
                write!(f, "proposer with index {} is not slashable", index)
            }
            InvalidProposerSlashing::InvalidSignature(bls) => {
                write!(f, "header has invalid signature: {:?}", bls)
            }
            InvalidProposerSlashing::InvalidIndex(index) => {
                write!(f, "proposer with index {} is not in state", index)
            }
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

#[derive(Debug)]
pub enum InvalidVoluntaryExit {
    InvalidIndex(ValidatorIndex),
    InactiveValidator(Epoch),
    ValidatorAlreadyExited {
        index: ValidatorIndex,
        epoch: Epoch,
    },
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
    InvalidParentHash {
        provided: Hash32,
        expected: Hash32,
    },
    InvalidPrevRandao {
        provided: Bytes32,
        expected: Bytes32,
    },
    InvalidTimestamp {
        provided: u64,
        expected: u64,
    },
}

impl Display for InvalidExecutionPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidExecutionPayload::InvalidParentHash { provided, expected } => write!(
                f,
                "expected parent hash {} but block has parent hash {}",
                expected, provided
            ),
            InvalidExecutionPayload::InvalidPrevRandao { provided, expected } => write!(
                f,
                "expected randao value {} but block has randao value {}",
                expected, provided
            ),
            InvalidExecutionPayload::InvalidTimestamp { provided, expected } => write!(
                f,
                "expected timestamp {} but block has timestamp {}",
                expected, provided
            ),
        }
    }
}

pub(crate) fn invalid_header_error(error: InvalidBeaconBlockHeader) -> Error {
    Error::InvalidBlock(Box::new(InvalidBlock::Header(error)))
}

pub(crate) fn invalid_operation_error(error: InvalidOperation) -> Error {
    Error::InvalidBlock(Box::new(InvalidBlock::InvalidOperation(error)))
}
