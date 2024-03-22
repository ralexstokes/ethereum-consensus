use crate::{
    runners::gen_exec,
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use ethereum_consensus::{primitives::Epoch, state_transition::Context};
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

fn run_test<S: Eq, T, B, C>(
    _pre: S,
    _expected: T,
    mut _pre_blocks: Vec<B>,
    mut _post_blocks: Vec<C>,
    meta: &Meta,
    _context: &Context,
) -> Result<(), Error> {
    todo!(
        "read for now to silence warning... {} {}",
        meta.post_fork,
        meta.fork_epoch,
        /*
        let mut context = $context.clone();
        let meta = $meta;
        match meta.post_fork.as_ref() {
            "altair" => {
                context.altair_fork_epoch = meta.fork_epoch;
                context.bellatrix_fork_epoch = Epoch::MAX;
            }
            "bellatrix" => {
                context.altair_fork_epoch = 0;
                context.bellatrix_fork_epoch = meta.fork_epoch;
                todo!("set other epochs?");
            }
            _ => todo!(),
        }
        let execution_engine = ethereum_consensus::bellatrix::DefaultExecutionEngine::default();
        let mut executor = Executor::<
            { spec::SLOTS_PER_HISTORICAL_ROOT },
            { spec::HISTORICAL_ROOTS_LIMIT },
            { spec::ETH1_DATA_VOTES_BOUND },
            { spec::VALIDATOR_REGISTRY_LIMIT },
            { spec::EPOCHS_PER_HISTORICAL_VECTOR },
            { spec::EPOCHS_PER_SLASHINGS_VECTOR },
            { spec::MAX_VALIDATORS_PER_COMMITTEE },
            { phase0_spec::PENDING_ATTESTATIONS_BOUND },
            { spec::SYNC_COMMITTEE_SIZE },
            { bellatrix_spec::BYTES_PER_LOGS_BLOOM },
            { bellatrix_spec::MAX_EXTRA_DATA_BYTES },
            { bellatrix_spec::MAX_BYTES_PER_TRANSACTION },
            { bellatrix_spec::MAX_TRANSACTIONS_PER_PAYLOAD },
            { spec::MAX_PROPOSER_SLASHINGS },
            { spec::MAX_ATTESTER_SLASHINGS },
            { spec::MAX_ATTESTATIONS },
            { spec::MAX_DEPOSITS },
            { spec::MAX_VOLUNTARY_EXITS },
            ethereum_consensus::bellatrix::DefaultExecutionEngine,
        >::new($pre.into(), execution_engine.into(), context);
        for block in $pre_blocks.into_iter() {
            let mut block = block.into();
            executor.apply_block(&mut block)?;
        }
        for block in $post_blocks.into_iter() {
            let mut block = block.into();
            executor.apply_block(&mut block)?;
        }
        todo!(
            /*
                match executor.state {
                    BeaconState::Altair(inner) => Ok(*inner),
                    _ => unreachable!(),
                }
                if pre == $post {
                    Ok(())
                } else {
                    Err(Error::InvalidState)
                }
             */
        )
        */
    )
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
                        | (pre, expected, pre_blocks, post_blocks, meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            run_test(pre, expected, pre_blocks, post_blocks, &meta, context)
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
                        | (pre, expected, pre_blocks, post_blocks, meta): (pre_spec::BeaconState, spec::BeaconState, Vec<pre_spec::SignedBeaconBlock>, Vec<spec::SignedBeaconBlock>, Meta), context: &Context| {
                            run_test(pre, expected, pre_blocks, post_blocks, &meta, context)
                        }
                    }
                }
                _ => todo!("implement after `run_test`"),
            },
            Config::Minimal => todo!("implement after `run_test`"),
            config => unreachable!("no tests for {config:?}"),
        },
        handler => unreachable!("no tests for {handler}"),
    }
}
