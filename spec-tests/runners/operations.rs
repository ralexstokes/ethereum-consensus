use crate::{
    runners::{gen_dispatch, gen_exec},
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use paste::paste;
use serde::Deserialize;

macro_rules! make_load_test {
    ($name: ident) => {
        paste! {
            fn [<load_ $name _test>] <S: ssz_rs::Deserialize, O: ssz_rs::Deserialize>(
                test_case_path: &str,
            ) -> (S, Option<S>, O) {
                let path = test_case_path.to_string() + "/pre.ssz_snappy";
                let pre: S = load_snappy_ssz(&path).unwrap();

                let path = test_case_path.to_string() + "/post.ssz_snappy";
                let post: Option<S> = load_snappy_ssz(&path);

                let path = test_case_path.to_string() + "/" + stringify!($name) + ".ssz_snappy";
                let operation: O = load_snappy_ssz(&path).unwrap();

                (pre, post, operation)
            }
        }
    };
}

make_load_test!(attestation);
make_load_test!(attester_slashing);
make_load_test!(block);
make_load_test!(deposit);
make_load_test!(proposer_slashing);
make_load_test!(voluntary_exit);
make_load_test!(sync_aggregate);

#[derive(Deserialize)]
struct ExecutionValidity {
    execution_valid: bool,
}

fn load_execution_payload_test<S: ssz_rs::Deserialize, O: ssz_rs::Deserialize>(
    test_case_path: &str,
) -> (S, Option<S>, O, bool) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: Option<S> = load_snappy_ssz(&path);

    let path = test_case_path.to_string() + "/execution_payload.ssz_snappy";
    let operation: O = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/execution.yaml";
    let execution_validity: ExecutionValidity = load_yaml(&path);

    (pre, post, operation, execution_validity.execution_valid)
}

macro_rules! run_test {
    ($context:expr, $pre:ident, $post:ident, $operation:ident, $exec_fn:ident) => {
        let mut pre = $pre;
        let result = spec::$exec_fn(&mut pre, $operation, $context);
        if let Some(post) = $post {
            assert_eq!(result.unwrap(), ());
            if pre == post {
                Ok(())
            } else {
                Err(Error::InvalidState)
            }
        } else {
            if result.is_err() {
                Ok(())
            } else {
                Err(Error::ExpectedError)
            }
        }
    };
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    let path = &test.data_path;
    match meta.handler.0.as_str() {
        "attestation" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_attestation_test::<spec::BeaconState, spec::Attestation>(path) },
                |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::Attestation), context| {
                    let operation = &operation;
                    run_test! { context, pre, post, operation, process_attestation }
                }
            }
        }
        "attester_slashing" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_attester_slashing_test::<spec::BeaconState, spec::AttesterSlashing>(path) },
                |(pre, post, mut operation): (spec::BeaconState, Option<spec::BeaconState>, spec::AttesterSlashing), context| {
                    let operation = &mut operation;
                    run_test! { context, pre, post, operation, process_attester_slashing }
                }
            }
        }
        "block_header" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_block_test::<spec::BeaconState, spec::BeaconBlock>(path) },
                |(pre, post, mut operation): (spec::BeaconState, Option<spec::BeaconState>, spec::BeaconBlock), context| {
                    let operation = &mut operation;
                    run_test! { context, pre, post, operation, process_block_header }
                }
            }
        }
        "deposit" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_deposit_test::<spec::BeaconState, spec::Deposit>(path) },
                |(pre, post, mut operation): (spec::BeaconState, Option<spec::BeaconState>, spec::Deposit), context| {
                    let operation = &mut operation;
                    run_test! { context, pre, post, operation, process_deposit }
                }
            }
        }
        "proposer_slashing" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_proposer_slashing_test::<spec::BeaconState, spec::ProposerSlashing>(path) },
                |(pre, post, mut operation): (spec::BeaconState, Option<spec::BeaconState>, spec::ProposerSlashing), context| {
                    let operation = &mut operation;
                    run_test! { context, pre, post, operation, process_proposer_slashing }
                }
            }
        }
        "voluntary_exit" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_voluntary_exit_test::<spec::BeaconState, spec::SignedVoluntaryExit>(path) },
                |(pre, post, mut operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SignedVoluntaryExit), context| {
                    let operation = &mut operation;
                    run_test! { context, pre, post, operation, process_voluntary_exit }
                }
            }
        }
        "sync_aggregate" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 => unreachable!("tests do not exist"),
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_sync_aggregate_test::<spec::BeaconState, spec::SyncAggregate>(path) },
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            let operation = &operation;
                            run_test! { context, pre, post, operation, process_sync_aggregate }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_sync_aggregate_test::<spec::BeaconState, spec::SyncAggregate>(path) },
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            let operation = &operation;
                            run_test! { context, pre, post, operation, process_sync_aggregate }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_sync_aggregate_test::<spec::BeaconState, spec::SyncAggregate>(path) },
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            let operation = &operation;
                            run_test! { context, pre, post, operation, process_sync_aggregate }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_sync_aggregate_test::<spec::BeaconState, spec::SyncAggregate>(path) },
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            let operation = &operation;
                            run_test! { context, pre, post, operation, process_sync_aggregate }
                        }
                    }
                }
            },
            Config::Minimal => match meta.fork {
                Fork::Phase0 => unreachable!("tests do not exist"),
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_sync_aggregate_test::<spec::BeaconState, spec::SyncAggregate>(path) },
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            let operation = &operation;
                            run_test! { context, pre, post, operation, process_sync_aggregate }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_sync_aggregate_test::<spec::BeaconState, spec::SyncAggregate>(path) },
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            let operation = &operation;
                            run_test! { context, pre, post, operation, process_sync_aggregate }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_sync_aggregate_test::<spec::BeaconState, spec::SyncAggregate>(path) },
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            let operation = &operation;
                            run_test! { context, pre, post, operation, process_sync_aggregate }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_sync_aggregate_test::<spec::BeaconState, spec::SyncAggregate>(path) },
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            let operation = &operation;
                            run_test! { context, pre, post, operation, process_sync_aggregate }
                        }
                    }
                }
            },
            _ => unreachable!(),
        },
        "execution_payload" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 | Fork::Altair => unreachable!("tests do not exist"),
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_execution_payload_test::<spec::BeaconState, spec::ExecutionPayload>(path) },
                        |(mut pre, post, mut operation, execution_valid): (spec::BeaconState, Option<spec::BeaconState>, spec::ExecutionPayload, bool), context| {
                            let engine = spec::DefaultExecutionEngine::new(execution_valid);
                            let result = spec::process_execution_payload(&mut pre, &mut operation, &engine, context);
                            if let Some(post) = post {
                                assert_eq!(result.unwrap(), ());
                                if pre == post {
                                    Ok(())
                                } else {
                                    Err(Error::InvalidState)
                                }
                            } else {
                                if result.is_err() {
                                    Ok(())
                                } else {
                                    Err(Error::ExpectedError)
                                }
                            }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_execution_payload_test::<spec::BeaconState, spec::ExecutionPayload>(path) },
                        |(mut pre, post, mut operation, execution_valid): (spec::BeaconState, Option<spec::BeaconState>, spec::ExecutionPayload, bool), context| {
                            let engine = spec::DefaultExecutionEngine::new(execution_valid);
                            let result = spec::process_execution_payload(&mut pre, &mut operation, &engine, context);
                            if let Some(post) = post {
                                assert_eq!(result.unwrap(), ());
                                if pre == post {
                                    Ok(())
                                } else {
                                    Err(Error::InvalidState)
                                }
                            } else {
                                if result.is_err() {
                                    Ok(())
                                } else {
                                    Err(Error::ExpectedError)
                                }
                            }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_execution_payload_test::<spec::BeaconState, spec::BeaconBlockBody>(path) },
                        |(mut pre, post, mut operation, execution_valid): (spec::BeaconState, Option<spec::BeaconState>, spec::BeaconBlockBody, bool), context| {
                            let engine = spec::DefaultExecutionEngine::new(execution_valid);
                            let result = spec::process_execution_payload(&mut pre, &mut operation, &engine, context);
                            if let Some(post) = post {
                                assert_eq!(result.unwrap(), ());
                                if pre == post {
                                    Ok(())
                                } else {
                                    Err(Error::InvalidState)
                                }
                            } else {
                                if result.is_err() {
                                    Ok(())
                                } else {
                                    Err(Error::ExpectedError)
                                }
                            }
                        }
                    }
                }
            },
            Config::Minimal => match meta.fork {
                Fork::Phase0 | Fork::Altair => unreachable!("tests do not exist"),
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_execution_payload_test::<spec::BeaconState, spec::ExecutionPayload>(path) },
                        |(mut pre, post, mut operation, execution_valid): (spec::BeaconState, Option<spec::BeaconState>, spec::ExecutionPayload, bool), context| {
                            let engine = spec::DefaultExecutionEngine::new(execution_valid);
                            let result = spec::process_execution_payload(&mut pre, &mut operation, &engine, context);
                            if let Some(post) = post {
                                assert_eq!(result.unwrap(), ());
                                if pre == post {
                                    Ok(())
                                } else {
                                    Err(Error::InvalidState)
                                }
                            } else {
                                if result.is_err() {
                                    Ok(())
                                } else {
                                    Err(Error::ExpectedError)
                                }
                            }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_execution_payload_test::<spec::BeaconState, spec::ExecutionPayload>(path) },
                        |(mut pre, post, mut operation, execution_valid): (spec::BeaconState, Option<spec::BeaconState>, spec::ExecutionPayload, bool), context| {
                            let engine = spec::DefaultExecutionEngine::new(execution_valid);
                            let result = spec::process_execution_payload(&mut pre, &mut operation, &engine, context);
                            if let Some(post) = post {
                                assert_eq!(result.unwrap(), ());
                                if pre == post {
                                    Ok(())
                                } else {
                                    Err(Error::InvalidState)
                                }
                            } else {
                                if result.is_err() {
                                    Ok(())
                                } else {
                                    Err(Error::ExpectedError)
                                }
                            }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_execution_payload_test::<spec::BeaconState, spec::BeaconBlockBody>(path) },
                        |(mut pre, post, mut operation, execution_valid): (spec::BeaconState, Option<spec::BeaconState>, spec::BeaconBlockBody, bool), context| {
                            let engine = spec::DefaultExecutionEngine::new(execution_valid);
                            let result = spec::process_execution_payload(&mut pre, &mut operation, &engine, context);
                            if let Some(post) = post {
                                assert_eq!(result.unwrap(), ());
                                if pre == post {
                                    Ok(())
                                } else {
                                    Err(Error::InvalidState)
                                }
                            } else {
                                if result.is_err() {
                                    Ok(())
                                } else {
                                    Err(Error::ExpectedError)
                                }
                            }
                        }
                    }
                }
            },
            _ => unreachable!(),
        },
        handler => Err(Error::UnknownHandler(handler.into(), meta.name())),
    }
}
