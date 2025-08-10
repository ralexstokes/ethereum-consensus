use ethereum_consensus::{
    altair, bellatrix, capella, deneb, phase0,
    state_transition::mainnet::{Context, Executor},
    types::{mainnet::SignedBeaconBlock, BeaconState},
};
use ssz_rs::prelude::*;
use std::{env, error::Error, fs, time::Instant};
#[derive(Debug)]
struct TransitionConfig {
    fork_name: &'static str,
    state_path: &'static str,
    start_slot: u64,
    end_slot: u64,
}
impl TransitionConfig {
    fn get_config(fork_name: &str) -> Option<TransitionConfig> {
        match fork_name {
            "phase0" => Some(TransitionConfig {
                fork_name: "phase0",
                state_path: "./ethereum-consensus/examples/mainnet_blocks/phase0/prestate/state_3199.ssz",
                start_slot: 3200,
                end_slot: 3231,
            }),
            "altair" => Some(TransitionConfig {
                fork_name: "altair",
                state_path: "./ethereum-consensus/examples/mainnet_blocks/altair/prestate/state_2375711.ssz",
                start_slot: 2375712,
                end_slot: 2375743,
            }),
            "bellatrix" => Some(TransitionConfig {
                fork_name: "bellatrix",
                state_path: "./ethereum-consensus/examples/mainnet_blocks/bellatrix/prestate/state_4636703.ssz",
                start_slot: 4636704,
                end_slot: 4636735,
            }),
            "capella" => Some(TransitionConfig {
                fork_name: "capella",
                state_path: "./ethereum-consensus/examples/mainnet_blocks/capella/prestate/state_6209567.ssz",
                start_slot: 6209568,
                end_slot: 6209599,
            }),
            "deneb" => Some(TransitionConfig {
                fork_name: "deneb",
                state_path: "./ethereum-consensus/examples/mainnet_blocks/deneb/prestate/state_8626207.ssz",
                start_slot: 8626208,
                end_slot: 8626239,
            }),
            _ => None,
        }
    }
}
// This clap processes an epoch's worth of mainnet beacon blocks against the CL specs, depending
// upon which fork you specify. Optimizations to the spec's STF are implemented behind feature
// flags.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <fork_name>", args[0]);
        println!("Available forks: phase0, altair, bellatrix, capella, deneb");
        return Ok(());
    }
    let fork_name = &args[1].to_lowercase();
    let config = TransitionConfig::get_config(fork_name).ok_or_else(|| {
        format!(
            "Unsupported fork: {}. Available forks: phase0, altair, bellatrix, capella, deneb",
            fork_name
        )
    })?;
    println!("Processing {} blocks...", config.fork_name);
    process_fork(config)?;
    Ok(())
}
fn process_fork(config: TransitionConfig) -> Result<(), Box<dyn Error>> {
    let state_bytes = fs::read(config.state_path)?;
    let context = Context::for_mainnet();
    let mut executor = match config.fork_name {
        "phase0" => {
            let state = phase0::mainnet::BeaconState::deserialize(&state_bytes)?;
            let prestate = BeaconState::Phase0(state);
            Executor::new(prestate, context)
        }
        "altair" => {
            let state = altair::mainnet::BeaconState::deserialize(&state_bytes)?;
            let prestate = BeaconState::Altair(state);
            Executor::new(prestate, context)
        }
        "bellatrix" => {
            let state = bellatrix::mainnet::BeaconState::deserialize(&state_bytes)?;
            let prestate = BeaconState::Bellatrix(state);
            Executor::new(prestate, context)
        }
        "capella" => {
            let state = capella::mainnet::BeaconState::deserialize(&state_bytes)?;
            let prestate = BeaconState::Capella(state);
            Executor::new(prestate, context)
        }
        "deneb" => {
            let state = deneb::mainnet::BeaconState::deserialize(&state_bytes)?;
            let prestate = BeaconState::Deneb(state);
            Executor::new(prestate, context)
        }
        _ => return Err("Unsupported fork".into()),
    };
    for slot in config.start_slot..=config.end_slot {
        let block_path = format!(
            "./ethereum-consensus/examples/mainnet_blocks/{}/beacon_blocks/block_{}.ssz",
            config.fork_name, slot
        );
        let block_bytes = fs::read(&block_path)?;
        if is_skipped_slot(&block_bytes, slot) {
            continue;
        }
        let block = match config.fork_name {
            "phase0" => {
                let signed_block = phase0::mainnet::SignedBeaconBlock::deserialize(&block_bytes)?;
                SignedBeaconBlock::Phase0(signed_block)
            }
            "altair" => {
                let signed_block = altair::mainnet::SignedBeaconBlock::deserialize(&block_bytes)?;
                SignedBeaconBlock::Altair(signed_block)
            }
            "bellatrix" => {
                let signed_block =
                    bellatrix::mainnet::SignedBeaconBlock::deserialize(&block_bytes)?;
                SignedBeaconBlock::Bellatrix(signed_block)
            }
            "capella" => {
                let signed_block = capella::mainnet::SignedBeaconBlock::deserialize(&block_bytes)?;
                SignedBeaconBlock::Capella(signed_block)
            }
            "deneb" => {
                let signed_block = deneb::mainnet::SignedBeaconBlock::deserialize(&block_bytes)?;
                SignedBeaconBlock::Deneb(signed_block)
            }
            _ => return Err("Unsupported fork".into()),
        };
        let start = Instant::now();
        println!("Begins processing block at slot {}", block.message().slot());
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
                println!();
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
