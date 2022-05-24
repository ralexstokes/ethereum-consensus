use ethereum_consensus::phase0::mainnet;
use ethereum_consensus::state_transition::{BeaconState, Context, Executor, SignedBeaconBlock};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let genesis_state = mainnet::BeaconState::default();
    let context = Context::for_mainnet(genesis_state.into());
    let mut executor = Executor::new(context);

    let mut signed_block = mainnet::SignedBeaconBlock::default();
    executor.apply_block(&mut signed_block.into())?;
    let root = executor.state_root()?;
    dbg!(root);
    Ok(())
}
