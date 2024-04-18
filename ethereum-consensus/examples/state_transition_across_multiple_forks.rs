use ethereum_consensus::{
    ssz::prelude::*,
    state_transition::mainnet::{Context, Executor},
    types::mainnet::{BeaconState, SignedBeaconBlock},
};
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    println!("this example illustrates how to use the codebase when applying state transitions");
    println!("note: the code currently does not run to the end as the input data is not correct");

    let genesis_state = BeaconState::Phase0(Default::default());
    let context = Context::for_mainnet();
    let mut executor = Executor::new(genesis_state, context);

    let mut block = SignedBeaconBlock::Phase0(Default::default());
    *block.message_mut().slot_mut() = 1;
    executor.apply_block(&block)?;

    let mut block = SignedBeaconBlock::Altair(Default::default());
    *block.message_mut().slot_mut() =
        executor.context.altair_fork_epoch * executor.context.slots_per_epoch;
    executor.apply_block(&block)?;

    let mut block = SignedBeaconBlock::Bellatrix(Default::default());
    *block.message_mut().slot_mut() =
        executor.context.bellatrix_fork_epoch * executor.context.slots_per_epoch;
    executor.apply_block(&block)?;

    let state = executor.state.bellatrix().unwrap();
    let state_root = state.hash_tree_root()?;
    dbg!(state_root);
    Ok(())
}
