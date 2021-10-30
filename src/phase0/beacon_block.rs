use crate::phase0::operations::{
    Attestation, AttesterSlashing, Deposit, ProposerSlashing, SignedVoluntaryExit,
};
use crate::phase0::Eth1Data;
use crate::{
    crypto::BLSSignature,
    presets::{
        MAX_ATTESTATIONS, MAX_ATTESTER_SLASHINGS, MAX_DEPOSITS, MAX_PROPOSER_SLASHINGS,
        MAX_VOLUNTARY_EXITS,
    },
    primitives::{Bytes32, Root, Slot, ValidatorIndex},
};
use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconBlockHeader {
    slot: Slot,
    proposer_index: ValidatorIndex,
    parent_root: Root,
    state_root: Root,
    body_root: Root,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SignedBeaconBlockHeader {
    message: BeaconBlockHeader,
    signature: BLSSignature,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconBlockBody {
    randao_reveal: BLSSignature,
    eth1_data: Eth1Data,
    graffiti: Bytes32,
    proposer_slashings: List<ProposerSlashing, MAX_PROPOSER_SLASHINGS>,
    attester_slashings: List<AttesterSlashing, MAX_ATTESTER_SLASHINGS>,
    attestations: List<Attestation, MAX_ATTESTATIONS>,
    deposits: List<Deposit, MAX_DEPOSITS>,
    voluntary_exits: List<SignedVoluntaryExit, MAX_VOLUNTARY_EXITS>,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct BeaconBlock {
    slot: Slot,
    proposer_index: ValidatorIndex,
    parent_root: Root,
    state_root: Root,
    body: BeaconBlockBody,
}

#[derive(Default, Debug, SimpleSerialize)]
pub struct SignedBeaconBlock {
    message: BeaconBlock,
    signature: BLSSignature,
}
