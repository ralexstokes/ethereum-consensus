mod beacon_block;
mod beacon_state;
mod operations;
mod validator;

use crate::crypto::{BLSPubkey, BLSSignature};
use crate::presets::{MAX_VALIDATORS_PER_COMMITTEE, SLOTS_PER_HISTORICAL_ROOT};
use crate::primitives::{
    Bytes32, CommitteeIndex, Domain, Epoch, Gwei, Hash32, Root, Slot, ValidatorIndex, Version,
};
use ssz_rs::prelude::*;

pub use beacon_block::*;
pub use beacon_state::*;
pub use operations::*;
pub use validator::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct Fork {
    previous_version: Version,
    current_version: Version,
    epoch: Epoch,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct ForkData {
    current_version: Version,
    genesis_validators_root: Root,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct Checkpoint {
    epoch: Epoch,
    root: Root,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct AttestationData {
    slot: Slot,
    index: CommitteeIndex,
    beacon_block_root: Root,
    source: Checkpoint,
    target: Checkpoint,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct IndexedAttestation {
    attesting_indices: List<ValidatorIndex, MAX_VALIDATORS_PER_COMMITTEE>,
    data: AttestationData,
    signature: BLSSignature,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct PendingAttestation {
    aggregation_bits: Bitlist<MAX_VALIDATORS_PER_COMMITTEE>,
    data: AttestationData,
    inclusion_delay: Slot,
    proposer_index: ValidatorIndex,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct Eth1Data {
    deposit_root: Root,
    deposit_count: u64,
    block_hash: Hash32,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct HistoricalBatch {
    block_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
    state_roots: Vector<Root, SLOTS_PER_HISTORICAL_ROOT>,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct DepositMessage {
    pubkey: BLSPubkey,
    withdrawal_credentials: Bytes32,
    amount: Gwei,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct DepositData {
    pubkey: BLSPubkey,
    withdrawal_credentials: Bytes32,
    amount: Gwei,
    signature: BLSSignature,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SigningData {
    object_root: Root,
    domain: Domain,
}
