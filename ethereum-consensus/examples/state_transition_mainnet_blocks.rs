use ethereum_consensus::{
    phase0::mainnet as spec,
    state_transition::mainnet::{Context, Executor},
    types::{mainnet::SignedBeaconBlock, BeaconState},
};
use ssz_rs::prelude::*;
use std::{error::Error, fs, time::Instant};

fn main() -> std::result::Result<(), Box<dyn Error>> {
    println!("this example illustrates how the spec applies state transitions to mainnet data.");

    let state_path = "./ethereum-consensus/examples/data/beacon_states/state_1999.ssz";
    let f = fs::read(state_path).unwrap();
    let prestate = spec::BeaconState::deserialize(&f)?;
    let prestate = BeaconState::Phase0(prestate);

    let context = Context::for_mainnet();
    let mut executor = Executor::new(prestate, context);

    // Read and process blocks at slots 2000-2034
    for slot in 2000..=2034 {
        let block_path =
            format!("./ethereum-consensus/examples/data/beacon_blocks/block_{}.ssz", slot);
        let block_bytes = fs::read(&block_path)?;
        if is_skipped_slot(&block_bytes, slot) {
            continue;
        }
        let signed_block = spec::SignedBeaconBlock::deserialize(&block_bytes)?;
        let block = SignedBeaconBlock::Phase0(signed_block);

        let start = Instant::now();
        executor.apply_block(&block)?;
        println!("Block at slot {slot} took {:?} to process", start.elapsed());
    }

    Ok(())
}

fn is_skipped_slot(block_bytes: &[u8], slot: u64) -> bool {
    if block_bytes.len() < 100 {
        match std::str::from_utf8(block_bytes) {
            Ok(text) if text.contains("NOT_FOUND") => {
                println!("Slot {} was skipped (no block produced)", slot);
                println!("\n");
                true
            }
            _ => {
                println!("Unexpected small file for slot {}", slot);
                true
            }
        }
    } else {
        false
    }
}
