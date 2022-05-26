// TODO: implement fork upgrade logic to demo a cross-fork transition
// use ethereum_consensus::altair::mainnet as altair;
// use ethereum_consensus::bellatrix::mainnet as bellatrix;
use ethereum_consensus::bellatrix::NoOpExecutionEngine;
use ethereum_consensus::phase0::mainnet as phase0;
use ethereum_consensus::state_transition::mainnet::{BeaconState, Context, Executor};
use ssz_rs::prelude::*;
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let genesis_state = phase0::genesis_state();
    let context = Context::for_mainnet();
    let mut executor = Executor::new(genesis_state.into(), NoOpExecutionEngine, context);

    let block = phase0::SignedBeaconBlock::default();
    executor.apply_block(&mut block.into())?;
    let state_root = match executor.state {
        BeaconState::Phase0(mut state) => state.hash_tree_root()?,
        BeaconState::Altair(mut state) => state.hash_tree_root()?,
        BeaconState::Bellatrix(mut state) => state.hash_tree_root()?,
    };
    dbg!(state_root);
    Ok(())
}
