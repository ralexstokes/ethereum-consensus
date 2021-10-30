use crate::crypto::BLSSignature;
use crate::phase0::beacon_block::SignedBeaconBlockHeader;
use crate::phase0::DepositData;
use crate::primitives::{Bytes32, Epoch, ValidatorIndex, DEPOSIT_CONTRACT_TREE_DEPTH};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct ProposerSlashing {
    pub signed_header_1: SignedBeaconBlockHeader,
    pub signed_header_2: SignedBeaconBlockHeader,
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
