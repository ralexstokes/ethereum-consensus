use crate::{
    runners::gen_exec,
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz, Error},
};
use ethereum_consensus::state_transition::Context;

fn load_test<S: ssz_rs::Deserialize, T: ssz_rs::Deserialize>(test_case_path: &str) -> (S, T) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: T = load_snappy_ssz(&path).unwrap();

    (pre, post)
}

fn run_test<S, T: Eq, F>(pre: S, expected: T, context: &Context, exec_fn: F) -> Result<(), Error>
where
    F: FnOnce(&S, &Context) -> T,
{
    let post = exec_fn(&pre, context);
    if expected == post {
        Ok(())
    } else {
        Err(Error::InvalidState)
    }
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "fork" => match test.meta.config {
            Config::Mainnet => match test.meta.fork {
                Fork::Altair => {
                    use ethereum_consensus::{
                        altair::mainnet as spec, phase0::mainnet as pre_spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            run_test(pre, expected, context, |state, context| spec::upgrade_to_altair(state, context).unwrap())
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
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            run_test(pre, expected, context, spec::upgrade_to_bellatrix)
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
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            run_test(pre, expected, context, spec::upgrade_to_capella)
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
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            run_test(pre, expected, context, spec::upgrade_to_deneb)
                        }
                    }
                }
                fork => unreachable!("no tests for (Mainnet, {fork:?})"),
            },
            Config::Minimal => match test.meta.fork {
                Fork::Altair => {
                    use ethereum_consensus::{
                        altair::minimal as spec, phase0::minimal as pre_spec,
                    };
                    gen_exec! {
                        test,
                        load_test,
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            run_test(pre, expected, context, |state, context| spec::upgrade_to_altair(state, context).unwrap())
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
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            run_test(pre, expected, context, spec::upgrade_to_bellatrix)
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
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            run_test(pre, expected, context, spec::upgrade_to_capella)
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
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            run_test(pre, expected, context, spec::upgrade_to_deneb)
                        }
                    }
                }
                fork => unreachable!("no tests for (Minimal, {fork:?})"),
            },
            config => unreachable!("no tests for {config:?}"),
        },
        handler => unreachable!("no tests for {handler}"),
    }
}
