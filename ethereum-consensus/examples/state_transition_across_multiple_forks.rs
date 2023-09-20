use ethereum_consensus::{
    altair::mainnet as altair,
    bellatrix::mainnet as bellatrix,
    phase0::mainnet as phase0,
    ssz::prelude::*,
    state_transition::mainnet::{Context, ExecutionEngine, Executor},
};
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    println!("this example is for illustration purposes...");
    println!("to get to the end, we need utilities to make correct blocks with respect to the state transition");

    let genesis_state = phase0::BeaconState::default();
    let context = Context::for_mainnet();
    let execution_engine = ExecutionEngine::Bellatrix(bellatrix::DefaultExecutionEngine::default());
    let mut executor = Executor::new(genesis_state.into(), execution_engine, context);

    let mut block = phase0::SignedBeaconBlock::default();
    block.message.slot = 1;
    executor.apply_block(&mut block.into())?;

    let mut block = altair::SignedBeaconBlock::default();
    block.message.slot = executor.context.altair_fork_epoch * executor.context.slots_per_epoch;
    executor.apply_block(&mut block.into())?;

    let mut block = bellatrix::SignedBeaconBlock::default();
    block.message.slot = executor.context.bellatrix_fork_epoch * executor.context.slots_per_epoch;
    executor.apply_block(&mut block.into())?;

    let mut state = executor.state.bellatrix().unwrap();
    let state_root = state.hash_tree_root()?;
    dbg!(state_root);
    Ok(())
}
