use crate::{
    runners::gen_exec,
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz, Error},
};

fn load_test<S: ssz_rs::Deserialize, T: ssz_rs::Deserialize>(test_case_path: &str) -> (S, T) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: T = load_snappy_ssz(&path).unwrap();

    (pre, post)
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    let path = &test.data_path;
    match meta.handler.0.as_str() {
        "fork" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Altair => {
                    use ethereum_consensus::{
                        altair::mainnet as spec, phase0::mainnet as pre_spec,
                    };
                    gen_exec! {
                        path,
                        meta.config,
                        |path| { load_test::<pre_spec::BeaconState, spec::BeaconState>(path)},
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            let post = spec::upgrade_to_altair(&pre, context).unwrap();
                            if expected == post {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Bellatrix => {
                    use ethereum_consensus::{
                        altair::mainnet as pre_spec, bellatrix::mainnet as spec,
                    };
                    gen_exec! {
                        path,
                        meta.config,
                        |path| { load_test::<pre_spec::BeaconState, spec::BeaconState>(path)},
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            let post = spec::upgrade_to_bellatrix(&pre, context);
                            if expected == post {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Capella => {
                    use ethereum_consensus::{
                        bellatrix::mainnet as pre_spec, capella::mainnet as spec,
                    };
                    gen_exec! {
                        path,
                        meta.config,
                        |path| { load_test::<pre_spec::BeaconState, spec::BeaconState>(path)},
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            let post = spec::upgrade_to_capella(&pre, context);
                            if expected == post {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Deneb => {
                    use ethereum_consensus::{
                        capella::mainnet as pre_spec, deneb::mainnet as spec,
                    };
                    gen_exec! {
                        path,
                        meta.config,
                        |path| { load_test::<pre_spec::BeaconState, spec::BeaconState>(path)},
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            let post = spec::upgrade_to_deneb(&pre, context);
                            if expected == post {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                _ => unreachable!(),
            },
            Config::Minimal => match meta.fork {
                Fork::Altair => {
                    use ethereum_consensus::{
                        altair::minimal as spec, phase0::minimal as pre_spec,
                    };
                    gen_exec! {
                        path,
                        meta.config,
                        |path| { load_test::<pre_spec::BeaconState, spec::BeaconState>(path)},
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            let post = spec::upgrade_to_altair(&pre, context).unwrap();
                            if expected == post {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Bellatrix => {
                    use ethereum_consensus::{
                        altair::minimal as pre_spec, bellatrix::minimal as spec,
                    };
                    gen_exec! {
                        path,
                        meta.config,
                        |path| { load_test::<pre_spec::BeaconState, spec::BeaconState>(path)},
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            let post = spec::upgrade_to_bellatrix(&pre, context);
                            if expected == post {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Capella => {
                    use ethereum_consensus::{
                        bellatrix::minimal as pre_spec, capella::minimal as spec,
                    };
                    gen_exec! {
                        path,
                        meta.config,
                        |path| { load_test::<pre_spec::BeaconState, spec::BeaconState>(path)},
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            let post = spec::upgrade_to_capella(&pre, context);
                            if expected == post {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Deneb => {
                    use ethereum_consensus::{
                        capella::minimal as pre_spec, deneb::minimal as spec,
                    };
                    gen_exec! {
                        path,
                        meta.config,
                        |path| { load_test::<pre_spec::BeaconState, spec::BeaconState>(path)},
                        |(pre, expected): (pre_spec::BeaconState, spec::BeaconState), context| {
                            let post = spec::upgrade_to_deneb(&pre, context);
                            if expected == post {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        handler => Err(Error::UnknownHandler(handler.into(), meta.name())),
    }
}
