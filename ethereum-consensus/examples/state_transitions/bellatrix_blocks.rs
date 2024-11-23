use ethereum_consensus::{
    bellatrix::mainnet as spec,
    state_transition::mainnet::{Context, Executor},
    types::{mainnet::SignedBeaconBlock, BeaconState},
};
use ssz_rs::prelude::*;
use std::{error::Error, fs, time::Instant};

fn main() -> std::result::Result<(), Box<dyn Error>> {
    println!(
        "this example illustrates how the spec applies state transitions to Bellatrix blocks."
    );

    let state_path =
        "./ethereum-consensus/examples/mainnet_blocks/bellatrix/prestate/state_4636703.ssz";
    let f = fs::read(state_path).unwrap();
    let prestate = spec::BeaconState::deserialize(&f)?;
    let prestate = BeaconState::Bellatrix(prestate);

    let context = Context::for_mainnet();
    let mut executor = Executor::new(prestate, context);

    // Read and process blocks at slots 4636704-4636735
    for slot in 4636704..=4636735 {
        let block_path = format!(
            "./ethereum-consensus/examples/mainnet_blocks/bellatrix/beacon_blocks/block_{}.ssz",
            slot
        );
        let block_bytes = fs::read(&block_path)?;
        if is_skipped_slot(&block_bytes, slot) {
            continue;
        }
        let signed_block = spec::SignedBeaconBlock::deserialize(&block_bytes)?;
        let block = SignedBeaconBlock::Bellatrix(signed_block);

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
