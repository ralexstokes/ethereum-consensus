use crate::{
    runners::gen_exec,
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz, Error},
};
use ethereum_consensus::primitives::Gwei;
use ssz_rs::prelude::*;

#[derive(Debug, Default, SimpleSerialize)]
struct Deltas<const VALIDATOR_REGISTRY_LIMIT: usize> {
    rewards: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
    penalties: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
}

type Pair = (Vec<Gwei>, Vec<Gwei>);

fn assert_deltas<const VALIDATOR_REGISTRY_LIMIT: usize>(
    expected: &Deltas<VALIDATOR_REGISTRY_LIMIT>,
    provided: Pair,
) {
    let rewards = provided.0;
    let penalties = provided.1;

    assert_eq!(rewards, expected.rewards.as_ref());
    assert_eq!(penalties, expected.penalties.as_ref());
}

fn load_test<const VALIDATOR_REGISTRY_LIMIT: usize, S: ssz_rs::Deserialize>(
    test_case_path: &str,
) -> (
    S,
    Deltas<VALIDATOR_REGISTRY_LIMIT>,
    Deltas<VALIDATOR_REGISTRY_LIMIT>,
    Deltas<VALIDATOR_REGISTRY_LIMIT>,
    Option<Deltas<VALIDATOR_REGISTRY_LIMIT>>,
    Deltas<VALIDATOR_REGISTRY_LIMIT>,
) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/source_deltas.ssz_snappy";
    let source_deltas: Deltas<VALIDATOR_REGISTRY_LIMIT> = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/target_deltas.ssz_snappy";
    let target_deltas: Deltas<VALIDATOR_REGISTRY_LIMIT> = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/head_deltas.ssz_snappy";
    let head_deltas: Deltas<VALIDATOR_REGISTRY_LIMIT> = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/inclusion_delay_deltas.ssz_snappy";
    let inclusion_delay_deltas: Option<Deltas<VALIDATOR_REGISTRY_LIMIT>> = load_snappy_ssz(&path);

    let path = test_case_path.to_string() + "/inactivity_penalty_deltas.ssz_snappy";
    let inactivity_penalty_deltas: Deltas<VALIDATOR_REGISTRY_LIMIT> =
        load_snappy_ssz(&path).unwrap();

    (
        pre,
        source_deltas,
        target_deltas,
        head_deltas,
        inclusion_delay_deltas,
        inactivity_penalty_deltas,
    )
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    let path = &test.data_path;
    match meta.handler.0.as_str() {
        "basic" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            expected_inclusion_delay_deltas,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let source_deltas = spec::get_source_deltas(&state, context).unwrap();
                            let target_deltas = spec::get_target_deltas(&state, context).unwrap();
                            let head_deltas = spec::get_head_deltas(&state, context).unwrap();
                            let inclusion_delay_deltas = spec::get_inclusion_delay_deltas(&state, context).unwrap();
                            let inactivity_penalty_deltas =
                                spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            if let Some(expected) = expected_inclusion_delay_deltas.as_ref() {
                                assert_deltas(expected, inclusion_delay_deltas);
                            }
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                _ => todo!(),
            },
            Config::Minimal => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            expected_inclusion_delay_deltas,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let source_deltas = spec::get_source_deltas(&state, context).unwrap();
                            let target_deltas = spec::get_target_deltas(&state, context).unwrap();
                            let head_deltas = spec::get_head_deltas(&state, context).unwrap();
                            let inclusion_delay_deltas = spec::get_inclusion_delay_deltas(&state, context).unwrap();
                            let inactivity_penalty_deltas =
                                spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            if let Some(expected) = expected_inclusion_delay_deltas.as_ref() {
                                assert_deltas(expected, inclusion_delay_deltas);
                            }
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                _ => todo!(),
            },
            _ => unreachable!(),
        },
        "leak" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            expected_inclusion_delay_deltas,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let source_deltas = spec::get_source_deltas(&state, context).unwrap();
                            let target_deltas = spec::get_target_deltas(&state, context).unwrap();
                            let head_deltas = spec::get_head_deltas(&state, context).unwrap();
                            let inclusion_delay_deltas = spec::get_inclusion_delay_deltas(&state, context).unwrap();
                            let inactivity_penalty_deltas =
                                spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            if let Some(expected) = expected_inclusion_delay_deltas.as_ref() {
                                assert_deltas(expected, inclusion_delay_deltas);
                            }
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                _ => todo!(),
            },
            Config::Minimal => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            expected_inclusion_delay_deltas,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let source_deltas = spec::get_source_deltas(&state, context).unwrap();
                            let target_deltas = spec::get_target_deltas(&state, context).unwrap();
                            let head_deltas = spec::get_head_deltas(&state, context).unwrap();
                            let inclusion_delay_deltas = spec::get_inclusion_delay_deltas(&state, context).unwrap();
                            let inactivity_penalty_deltas =
                                spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            if let Some(expected) = expected_inclusion_delay_deltas.as_ref() {
                                assert_deltas(expected, inclusion_delay_deltas);
                            }
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                _ => todo!(),
            },
            _ => unreachable!(),
        },
        "random" => match meta.config {
            Config::Mainnet => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            expected_inclusion_delay_deltas,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let source_deltas = spec::get_source_deltas(&state, context).unwrap();
                            let target_deltas = spec::get_target_deltas(&state, context).unwrap();
                            let head_deltas = spec::get_head_deltas(&state, context).unwrap();
                            let inclusion_delay_deltas = spec::get_inclusion_delay_deltas(&state, context).unwrap();
                            let inactivity_penalty_deltas =
                                spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            if let Some(expected) = expected_inclusion_delay_deltas.as_ref() {
                                assert_deltas(expected, inclusion_delay_deltas);
                            }
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                _ => todo!(),
            },
            Config::Minimal => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            expected_inclusion_delay_deltas,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let source_deltas = spec::get_source_deltas(&state, context).unwrap();
                            let target_deltas = spec::get_target_deltas(&state, context).unwrap();
                            let head_deltas = spec::get_head_deltas(&state, context).unwrap();
                            let inclusion_delay_deltas = spec::get_inclusion_delay_deltas(&state, context).unwrap();
                            let inactivity_penalty_deltas =
                                spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            if let Some(expected) = expected_inclusion_delay_deltas.as_ref() {
                                assert_deltas(expected, inclusion_delay_deltas);
                            }
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_test::<{ spec::VALIDATOR_REGISTRY_LIMIT }, spec::BeaconState>(path) },
                        |(
                            state,
                            expected_source_deltas,
                            expected_target_deltas,
                            expected_head_deltas,
                            _,
                            expected_inactivity_penalty_deltas,
                        ): (
                            spec::BeaconState,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                            Option<Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>>,
                            Deltas<{ spec::VALIDATOR_REGISTRY_LIMIT }>,
                        ),
                         context| {
                            let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                            let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                            let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                            let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                            let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();

                            assert_deltas(&expected_source_deltas, source_deltas);
                            assert_deltas(&expected_target_deltas, target_deltas);
                            assert_deltas(&expected_head_deltas, head_deltas);
                            assert_deltas(&expected_inactivity_penalty_deltas, inactivity_penalty_deltas);
                            Ok(())
                        }
                    }
                }
                _ => todo!(),
            },
            _ => unreachable!(),
        },
        handler => Err(Error::UnknownHandler(handler.into(), meta.name())),
    }
}
