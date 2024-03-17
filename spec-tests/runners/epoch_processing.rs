use crate::{
    runners::{gen_dispatch, gen_exec},
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz, Error},
};

fn load_test<S: ssz_rs::Deserialize>(test_case_path: &str) -> (S, Option<S>) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: Option<S> = load_snappy_ssz(&path);

    (pre, post)
}

macro_rules! run_test {
    ($context:expr, $pre:ident, $post:ident, $exec_fn:ident) => {
        let mut pre = $pre;
        let post = $post.expect("some post state");
        spec::$exec_fn(&mut pre, $context);
        if pre != post {
            Err(Error::InvalidState)
        } else {
            Ok(())
        }
    };
    ($context:expr, $pre:ident, $post: ident, $exec_fn:expr) => {
        let mut pre = $pre;
        let result = $exec_fn(&mut pre, $context);
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
        "effective_balance_updates" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, process_effective_balance_updates}
                }
            }
        }
        "eth1_data_reset" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, process_eth1_data_reset}
                }
            }
        }
        "historical_roots_update" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, |state, context| {
                        spec::process_historical_roots_update(state, context)
                    }}
                }
            }
        }
        "justification_and_finalization" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, |state, context| {
                        spec::process_justification_and_finalization(state, context)
                    }}
                }
            }
        }
        "participation_record_updates" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| {
                                spec::process_participation_record_updates(state);
                                Ok::<_, Error>(())
                            }}
                        }
                    }
                }
                _ => unreachable!("tests do not exist"),
            },
            Config::Minimal => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| {
                                spec::process_participation_record_updates(state);
                                Ok::<_, Error>(())
                            }}
                        }
                    }
                }
                _ => unreachable!("tests do not exist"),
            },
            _ => unreachable!(),
        },
        "randao_mixes_reset" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, process_randao_mixes_reset }
                }
            }
        }
        "registry_updates" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, process_registry_updates }
                }
            }
        }
        "rewards_and_penalties" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, |state, context| { spec::process_rewards_and_penalties(state, context) } }
                }
            }
        }
        "slashings" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, |state, context| {spec::process_slashings(state, context) } }
                }
            }
        }
        "slashings_reset" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState>(path) },
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test! { context, pre, post, process_slashings_reset }
                }
            }
        }
        "inactivity_updates" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 => unreachable!("tests do not exist"),
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_inactivity_updates(state, context) } }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| {spec::process_inactivity_updates(state, context) } }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_inactivity_updates(state, context) } }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_inactivity_updates(state, context) } }
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
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| {spec::process_inactivity_updates(state, context) } }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| {spec::process_inactivity_updates(state, context) } }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| {spec::process_inactivity_updates(state, context) } }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| {spec::process_inactivity_updates(state, context) } }
                        }
                    }
                }
            },
            _ => unreachable!(),
        },
        "participation_flag_updates" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 => unreachable!("tests do not exist"),
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| { spec::process_participation_flag_updates(state) } }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| { spec::process_participation_flag_updates(state) } }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| { spec::process_participation_flag_updates(state) } }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| { spec::process_participation_flag_updates(state) } }
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
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| { spec::process_participation_flag_updates(state) } }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| { spec::process_participation_flag_updates(state) } }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| { spec::process_participation_flag_updates(state) } }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, _| { spec::process_participation_flag_updates(state) } }
                        }
                    }
                }
            },
            _ => unreachable!(),
        },
        "sync_committee_updates" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 => unreachable!("tests do not exist"),
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_sync_committee_updates(state, context) } }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| {spec::process_sync_committee_updates(state, context) } }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_sync_committee_updates(state, context) } }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_sync_committee_updates(state, context) } }
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
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_sync_committee_updates(state, context) } }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| {spec::process_sync_committee_updates(state, context) } }
                        }
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_sync_committee_updates(state, context) } }
                        }
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<spec::BeaconState>(path) },
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test! { context, pre, post, |state, context| { spec::process_sync_committee_updates(state, context) } }
                        }
                    }
                }
            },
            _ => unreachable!(),
        },
        handler => Err(Error::UnknownHandler(handler.into(), meta.name())),
    }
}
