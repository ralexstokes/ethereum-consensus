use crate::crypto::BLSSignature;
use crate::phase0::beacon_block::SignedBeaconBlockHeader;
use crate::phase0::{AttestationData, DepositData, IndexedAttestation};
use crate::presets::MAX_VALIDATORS_PER_COMMITTEE;
use crate::primitives::{Bytes32, Epoch, ValidatorIndex, DEPOSIT_CONTRACT_TREE_DEPTH};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct ProposerSlashing {
    signed_header_1: SignedBeaconBlockHeader,
    signed_header_2: SignedBeaconBlockHeader,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct AttesterSlashing {
    attestation_1: IndexedAttestation,
    attestation_2: IndexedAttestation,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct Attestation {
    aggregation_bits: Bitlist<MAX_VALIDATORS_PER_COMMITTEE>,
    data: AttestationData,
    signature: BLSSignature,
}

const fn get_deposit_proof_length() -> usize {
    DEPOSIT_CONTRACT_TREE_DEPTH + 1
}

const DEPOSIT_PROOF_LENGTH: usize = get_deposit_proof_length();

#[derive(Default, Debug, SimpleSerialize)]
pub struct Deposit {
    proof: Vector<Bytes32, DEPOSIT_PROOF_LENGTH>,
    data: DepositData,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct VoluntaryExit {
    epoch: Epoch,
    validator_index: ValidatorIndex,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SignedVoluntaryExit {
    message: VoluntaryExit,
    signature: BLSSignature,
}
