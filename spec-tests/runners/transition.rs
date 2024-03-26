use crate::{
    runners::gen_exec,
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use ethereum_consensus::{
    primitives::Epoch,
    state_transition::{self, Context},
    types::{BeaconState, SignedBeaconBlock},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Meta {
    post_fork: String,
    fork_epoch: Epoch,
    fork_block: Option<usize>,
    blocks_count: usize,
}

fn load_test<
    S: ssz_rs::Deserialize,
    T: ssz_rs::Deserialize,
    B: ssz_rs::Deserialize,
    C: ssz_rs::Deserialize,
>(
    test_case_path: &str,
) -> (S, T, Vec<B>, Vec<C>, Meta) {
    let path = test_case_path.to_string() + "/meta.yaml";
    let meta: Meta = load_yaml(&path);

    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let blocks_count = meta.blocks_count;
    let mut pre_blocks = vec![];
    let mut post_blocks = vec![];
    for i in 0..blocks_count {
        let path = format!("{}/blocks_{}.ssz_snappy", test_case_path, i);
        if let Some(fork_index) = meta.fork_block {
            if i <= fork_index {
                let block: B = load_snappy_ssz(&path).unwrap();
                pre_blocks.push(block);
            } else {
                let block: C = load_snappy_ssz(&path).unwrap();
                post_blocks.push(block);
            }
        } else {
            let block: C = load_snappy_ssz(&path).unwrap();
            post_blocks.push(block);
        }
    }

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: T = load_snappy_ssz(&path).unwrap();

    (pre, post, pre_blocks, post_blocks, meta)
}

fn set_fork_epochs(meta: &mut Meta, context: &mut Context) {
    context.altair_fork_epoch = Epoch::MAX;
    context.bellatrix_fork_epoch = Epoch::MAX;
    context.capella_fork_epoch = Epoch::MAX;
    context.deneb_fork_epoch = Epoch::MAX;

    match meta.post_fork.as_ref() {
        "altair" => {
            context.altair_fork_epoch = meta.fork_epoch;
        }
        "bellatrix" => {
            context.altair_fork_epoch = 0;
            context.bellatrix_fork_epoch = meta.fork_epoch;
        }
        "capella" => {
            context.altair_fork_epoch = 0;
            context.bellatrix_fork_epoch = 0;
            context.capella_fork_epoch = meta.fork_epoch;
        }
        "deneb" => {
            context.altair_fork_epoch = 0;
            context.bellatrix_fork_epoch = 0;
            context.capella_fork_epoch = 0;
            context.deneb_fork_epoch = meta.fork_epoch;
        }
        _ => todo!(),
    }
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "core" => match test.meta.config {
            Config::Mainnet => match test.meta.fork {
                Fork::Altair => {
                    use ethereum_consensus::{
                        altair::mainnet as spec, phase0::mainnet as pre_spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        | (pre, expected, pre_blocks, post_blocks, mut meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            assert_eq!(meta.post_fork, "altair");
                            let mut context = context.clone();
                            set_fork_epochs(&mut meta, &mut context);
                            let mut executor = state_transition::mainnet::Executor::new(BeaconState::Phase0(pre), context);
                            for block in pre_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Phase0(block);
                                executor.apply_block(&mut block)?;
                            }
                            for block in post_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Altair(block);
                                executor.apply_block(&mut block)?;
                            }
                            let post = executor.state.altair().unwrap();
                            if post != &expected {
                                Err(Error::InvalidState)
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
                Fork::Bellatrix => {
                    use ethereum_consensus::{
                        altair::mainnet as pre_spec, bellatrix::mainnet as spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        | (pre, expected, pre_blocks, post_blocks, mut meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            assert_eq!(meta.post_fork, "bellatrix");
                            let mut context = context.clone();
                            set_fork_epochs(&mut meta, &mut context);
                            let mut executor = state_transition::mainnet::Executor::new(BeaconState::Altair(pre), context);
                            for block in pre_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Altair(block);
                                executor.apply_block(&mut block)?;
                            }
                            for block in post_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Bellatrix(block);
                                executor.apply_block(&mut block)?;
                            }
                            let post = executor.state.bellatrix().unwrap();
                            if post != &expected {
                                Err(Error::InvalidState)
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
                Fork::Capella => {
                    use ethereum_consensus::{
                        bellatrix::mainnet as pre_spec, capella::mainnet as spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        | (pre, expected, pre_blocks, post_blocks, mut meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            assert_eq!(meta.post_fork, "capella");
                            let mut context = context.clone();
                            set_fork_epochs(&mut meta, &mut context);
                            let mut executor = state_transition::mainnet::Executor::new(BeaconState::Bellatrix(pre), context);
                            for block in pre_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Bellatrix(block);
                                executor.apply_block(&mut block)?;
                            }
                            for block in post_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Capella(block);
                                executor.apply_block(&mut block)?;
                            }
                            let post = executor.state.capella().unwrap();
                            if post != &expected {
                                Err(Error::InvalidState)
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
                Fork::Deneb => {
                    use ethereum_consensus::{
                        capella::mainnet as pre_spec, deneb::mainnet as spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        | (pre, expected, pre_blocks, post_blocks, mut meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            assert_eq!(meta.post_fork, "deneb");
                            let mut context = context.clone();
                            set_fork_epochs(&mut meta, &mut context);
                            let mut executor = state_transition::mainnet::Executor::new(BeaconState::Capella(pre), context);
                            for block in pre_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Capella(block);
                                executor.apply_block(&mut block)?;
                            }
                            for block in post_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Deneb(block);
                                executor.apply_block(&mut block)?;
                            }
                            let post = executor.state.deneb().unwrap();
                            if post != &expected {
                                Err(Error::InvalidState)
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
                _ => todo!(),
            },
            Config::Minimal => match test.meta.fork {
                Fork::Altair => {
                    use ethereum_consensus::{
                        altair::minimal as spec, phase0::minimal as pre_spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        | (pre, expected, pre_blocks, post_blocks, mut meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            assert_eq!(meta.post_fork, "altair");
                            let mut context = context.clone();
                            set_fork_epochs(&mut meta, &mut context);
                            let mut executor = state_transition::minimal::Executor::new(BeaconState::Phase0(pre), context);
                            for block in pre_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Phase0(block);
                                executor.apply_block(&mut block)?;
                            }
                            for block in post_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Altair(block);
                                executor.apply_block(&mut block)?;
                            }
                            let post = executor.state.altair().unwrap();
                            if post != &expected {
                                Err(Error::InvalidState)
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
                Fork::Bellatrix => {
                    use ethereum_consensus::{
                        altair::minimal as pre_spec, bellatrix::minimal as spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        | (pre, expected, pre_blocks, post_blocks, mut meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            assert_eq!(meta.post_fork, "bellatrix");
                            let mut context = context.clone();
                            set_fork_epochs(&mut meta, &mut context);
                            let mut executor = state_transition::minimal::Executor::new(BeaconState::Altair(pre), context);
                            for block in pre_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Altair(block);
                                executor.apply_block(&mut block)?;
                            }
                            for block in post_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Bellatrix(block);
                                executor.apply_block(&mut block)?;
                            }
                            let post = executor.state.bellatrix().unwrap();
                            if post != &expected {
                                Err(Error::InvalidState)
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
                Fork::Capella => {
                    use ethereum_consensus::{
                        bellatrix::minimal as pre_spec, capella::minimal as spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        | (pre, expected, pre_blocks, post_blocks, mut meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            assert_eq!(meta.post_fork, "capella");
                            let mut context = context.clone();
                            set_fork_epochs(&mut meta, &mut context);
                            let mut executor = state_transition::minimal::Executor::new(BeaconState::Bellatrix(pre), context);
                            for block in pre_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Bellatrix(block);
                                executor.apply_block(&mut block)?;
                            }
                            for block in post_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Capella(block);
                                executor.apply_block(&mut block)?;
                            }
                            let post = executor.state.capella().unwrap();
                            if post != &expected {
                                Err(Error::InvalidState)
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
                Fork::Deneb => {
                    use ethereum_consensus::{
                        capella::minimal as pre_spec, deneb::minimal as spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        | (pre, expected, pre_blocks, post_blocks, mut meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            assert_eq!(meta.post_fork, "deneb");
                            let mut context = context.clone();
                            set_fork_epochs(&mut meta, &mut context);
                            let mut executor = state_transition::minimal::Executor::new(BeaconState::Capella(pre), context);
                            for block in pre_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Capella(block);
                                executor.apply_block(&mut block)?;
                            }
                            for block in post_blocks.into_iter() {
                                let mut block = SignedBeaconBlock::Deneb(block);
                                executor.apply_block(&mut block)?;
                            }
                            let post = executor.state.deneb().unwrap();
                            if post != &expected {
                                Err(Error::InvalidState)
                            } else {
                                Ok(())
                            }
                        }
                    }
                }
                _ => todo!(),
            },
            config => unreachable!("no tests for {config:?}"),
        },
        handler => unreachable!("no tests for {handler}"),
    }
}
