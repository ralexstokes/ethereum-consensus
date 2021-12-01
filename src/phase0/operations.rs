use crate::crypto::{PublicKey as BLSPubkey, Signature as BLSSignature};
use crate::phase0::beacon_block::SignedBeaconBlockHeader;
use crate::phase0::DEPOSIT_CONTRACT_TREE_DEPTH;
use crate::primitives::{Bytes32, CommitteeIndex, Epoch, Gwei, Hash32, Root, Slot, ValidatorIndex};
use ssz_rs::prelude::*;

#[derive(Default, Clone, Debug, SimpleSerialize, PartialEq, Eq)]
pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: Root,
}

#[derive(Default, Clone, Debug, SimpleSerialize, PartialEq, Eq)]
pub struct AttestationData {
    pub slot: Slot,
    pub index: CommitteeIndex,
    pub beacon_block_root: Root,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct IndexedAttestation<const MAX_VALIDATORS_PER_COMMITTEE: usize> {
    pub attesting_indices: List<ValidatorIndex, MAX_VALIDATORS_PER_COMMITTEE>,
    pub data: AttestationData,
    pub signature: BLSSignature,
}

#[derive(Default, Debug, SimpleSerialize, Clone)]
pub struct PendingAttestation<const MAX_VALIDATORS_PER_COMMITTEE: usize> {
    pub aggregation_bits: Bitlist<MAX_VALIDATORS_PER_COMMITTEE>,
    pub data: AttestationData,
    pub inclusion_delay: Slot,
    pub proposer_index: ValidatorIndex,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct Attestation<const MAX_VALIDATORS_PER_COMMITTEE: usize> {
    pub aggregation_bits: Bitlist<MAX_VALIDATORS_PER_COMMITTEE>,
    pub data: AttestationData,
    pub signature: BLSSignature,
}

#[derive(Default, Debug, SimpleSerialize, Clone)]
pub struct Eth1Data {
    pub deposit_root: Root,
    pub deposit_count: u64,
    pub block_hash: Hash32,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct DepositMessage {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: Bytes32,
    pub amount: Gwei,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct DepositData {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: Bytes32,
    pub amount: Gwei,
    pub signature: BLSSignature,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct ProposerSlashing {
    pub signed_header_1: SignedBeaconBlockHeader,
    pub signed_header_2: SignedBeaconBlockHeader,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct AttesterSlashing<const MAX_VALIDATORS_PER_COMMITTEE: usize> {
    pub attestation_1: IndexedAttestation<MAX_VALIDATORS_PER_COMMITTEE>,
    pub attestation_2: IndexedAttestation<MAX_VALIDATORS_PER_COMMITTEE>,
}

const fn get_deposit_proof_length() -> usize {
    DEPOSIT_CONTRACT_TREE_DEPTH + 1
}

const DEPOSIT_PROOF_LENGTH: usize = get_deposit_proof_length();

#[derive(Default, Debug, SimpleSerialize)]
pub struct Deposit {
    pub proof: Vector<Bytes32, DEPOSIT_PROOF_LENGTH>,
    pub data: DepositData,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct VoluntaryExit {
    pub epoch: Epoch,
    pub validator_index: ValidatorIndex,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: BLSSignature,
}
