use crate::{
    runners::{gen_exec, gen_match_for, gen_match_for_all},
    test_case::TestCase,
    test_utils::{load_snappy_ssz, Error},
};
use ethereum_consensus::{state_transition::Context, Error as SpecError};

fn load_test<S: ssz_rs::Deserialize>(test_case_path: &str) -> (S, Option<S>) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: Option<S> = load_snappy_ssz(&path);

    (pre, post)
}

fn run_test<S: Eq, F>(
    mut pre: S,
    post: Option<S>,
    context: &Context,
    exec_fn: F,
) -> Result<(), Error>
where
    F: FnOnce(&mut S, &Context) -> Result<(), SpecError>,
{
    let result = exec_fn(&mut pre, context);
    if let Some(post) = post {
        assert_eq!(result.unwrap(), ());
        if pre != post {
            Err(Error::InvalidState)
        } else {
            Ok(())
        }
    } else {
        if result.is_ok() {
            Err(Error::ExpectedError)
        } else {
            Ok(())
        }
    }
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "effective_balance_updates" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, |state, context| {
                        spec::process_effective_balance_updates(state, context);
                        Ok(())
                    })
                }
            }
        }
        "eth1_data_reset" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, |state, context| {
                        spec::process_eth1_data_reset(state, context);
                        Ok(())
                    })
                }
            }
        }
        "historical_roots_update" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, spec::process_historical_roots_update)
                }
            }
        }
        "justification_and_finalization" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, spec::process_justification_and_finalization)
                }
            }
        }
        "participation_record_updates" => {
            gen_match_for! {
                test,
                (mainnet, phase0),
                (minimal, phase0)
                {
                    gen_exec! {
                        test,
                        load_test,
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test(pre, post, context, |state, _| {
                                spec::process_participation_record_updates(state);
                                Ok(())
                            })
                        }
                    }
                }
            }
        }
        "randao_mixes_reset" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, |state, context| {
                        spec::process_randao_mixes_reset(state, context);
                        Ok(())
                    })
                }
            }
        }
        "registry_updates" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, |state, context| {
                        spec::process_registry_updates(state, context);
                        Ok(())
                    })
                }
            }
        }
        "rewards_and_penalties" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, spec::process_rewards_and_penalties)
                }
            }
        }
        "slashings" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, spec::process_slashings)
                }
            }
        }
        "slashings_reset" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                    run_test(pre, post, context, |state, context| {
                        spec::process_slashings_reset(state, context);
                        Ok(())
                    })
                }
            }
        }
        "inactivity_updates" => {
            gen_match_for! {
                test,
                (mainnet, altair),
                (mainnet, bellatrix),
                (mainnet, capella),
                (mainnet, deneb),
                (minimal, altair),
                (minimal, bellatrix),
                (minimal, capella),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_test,
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test(pre, post, context, spec::process_inactivity_updates)
                        }
                    }
                }
            }
        }
        "participation_flag_updates" => {
            gen_match_for! {
                test,
                (mainnet, altair),
                (mainnet, bellatrix),
                (mainnet, capella),
                (mainnet, deneb),
                (minimal, altair),
                (minimal, bellatrix),
                (minimal, capella),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_test,
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test(pre, post, context, |state, _| {
                                spec::process_participation_flag_updates(state)
                            })
                        }
                    }
                }
            }
        }
        "sync_committee_updates" => {
            gen_match_for! {
                test,
                (mainnet, altair),
                (mainnet, bellatrix),
                (mainnet, capella),
                (mainnet, deneb),
                (minimal, altair),
                (minimal, bellatrix),
                (minimal, capella),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_test,
                        |(pre, post): (spec::BeaconState, Option<spec::BeaconState>), context| {
                            run_test(pre, post, context, spec::process_sync_committee_updates)
                        }
                    }
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
