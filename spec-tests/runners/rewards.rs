use crate::{
    runners::{gen_exec, gen_match_for},
    test_case::TestCase,
    test_utils::{load_snappy_ssz, Error},
};
use ethereum_consensus::{primitives::Gwei, state_transition::Context};
use ssz_rs::prelude::*;

#[derive(Debug, Default, SimpleSerialize)]
struct Deltas<const VALIDATOR_REGISTRY_LIMIT: usize> {
    rewards: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
    penalties: List<Gwei, VALIDATOR_REGISTRY_LIMIT>,
}

type Pair = (Vec<Gwei>, Vec<Gwei>);

type RewardsDeltas<const VALIDATOR_REGISTRY_LIMIT: usize> = (
    Deltas<VALIDATOR_REGISTRY_LIMIT>,
    Deltas<VALIDATOR_REGISTRY_LIMIT>,
    Deltas<VALIDATOR_REGISTRY_LIMIT>,
    Option<Deltas<VALIDATOR_REGISTRY_LIMIT>>,
    Deltas<VALIDATOR_REGISTRY_LIMIT>,
);

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
) -> (S, RewardsDeltas<VALIDATOR_REGISTRY_LIMIT>) {
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
        (
            source_deltas,
            target_deltas,
            head_deltas,
            inclusion_delay_deltas,
            inactivity_penalty_deltas,
        ),
    )
}

fn run_test<const VALIDATOR_REGISTRY_LIMIT: usize, S, F>(
    state: &S,
    context: &Context,
    expected: RewardsDeltas<VALIDATOR_REGISTRY_LIMIT>,
    exec_fn: F,
) -> Result<(), Error>
where
    F: FnOnce(&S, &Context) -> (Pair, Pair, Pair, Option<Pair>, Pair),
{
    let (
        source_deltas,
        target_deltas,
        head_deltas,
        inclusion_delay_deltas,
        inactivity_penalty_deltas,
    ) = exec_fn(state, context);
    assert_deltas(&expected.0, source_deltas);
    assert_deltas(&expected.1, target_deltas);
    assert_deltas(&expected.2, head_deltas);
    if let Some(expected) = expected.3.as_ref() {
        assert_deltas(expected, inclusion_delay_deltas.unwrap());
    }
    assert_deltas(&expected.4, inactivity_penalty_deltas);
    Ok(())
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "basic" | "leak" | "random" => {
            gen_match_for! {
                test,
                (mainnet, phase0) => {
                    gen_exec! {
                        test,
                        load_test,
                        |(state, expected): (spec::BeaconState, RewardsDeltas<{spec::VALIDATOR_REGISTRY_LIMIT}>), context| {
                            run_test(&state, context, expected, |state, context| {
                                let source_deltas = spec::get_source_deltas(&state, context).unwrap();
                                let target_deltas = spec::get_target_deltas(&state, context).unwrap();
                                let head_deltas = spec::get_head_deltas(&state, context).unwrap();
                                let inclusion_delay_deltas = spec::get_inclusion_delay_deltas(&state, context).unwrap();
                                let inactivity_penalty_deltas =
                                    spec::get_inactivity_penalty_deltas(&state, context).unwrap();
                                (source_deltas, target_deltas, head_deltas, Some(inclusion_delay_deltas), inactivity_penalty_deltas)
                            })
                        }
                    }
                }
                (minimal, phase0) => {
                    gen_exec! {
                        test,
                        load_test,
                        |(state, expected): (spec::BeaconState, RewardsDeltas<{spec::VALIDATOR_REGISTRY_LIMIT}>), context| {
                            run_test(&state, context, expected, |state, context| {
                                let source_deltas = spec::get_source_deltas(&state, context).unwrap();
                                let target_deltas = spec::get_target_deltas(&state, context).unwrap();
                                let head_deltas = spec::get_head_deltas(&state, context).unwrap();
                                let inclusion_delay_deltas = spec::get_inclusion_delay_deltas(&state, context).unwrap();
                                let inactivity_penalty_deltas =
                                    spec::get_inactivity_penalty_deltas(&state, context).unwrap();
                                (source_deltas, target_deltas, head_deltas, Some(inclusion_delay_deltas), inactivity_penalty_deltas)
                            })
                        }
                    }
                }
                (mainnet, altair) => {
                    gen_exec! {
                        test,
                        load_test,
                        |(state, expected): (spec::BeaconState, RewardsDeltas<{spec::VALIDATOR_REGISTRY_LIMIT}>), context| {
                            run_test(&state, context, expected, |state, context| {
                                let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                                let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                                let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                                let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();
                                (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                            })
                        }
                    }
                }
                (mainnet, bellatrix) => {
                    gen_exec! {
                        test,
                        load_test,
                        |(state, expected): (spec::BeaconState, RewardsDeltas<{spec::VALIDATOR_REGISTRY_LIMIT}>), context| {
                            run_test(&state, context, expected, |state, context| {
                                let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                                let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                                let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                                let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();
                                (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                            })
                        }
                    }
                }
                (minimal, altair) => {
                    gen_exec! {
                        test,
                        load_test,
                        |(state, expected): (spec::BeaconState, RewardsDeltas<{spec::VALIDATOR_REGISTRY_LIMIT}>), context| {
                            run_test(&state, context, expected, |state, context| {
                                let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                                let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                                let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                                let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();
                                (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                            })
                        }
                    }
                }
                (minimal, bellatrix) => {
                    gen_exec! {
                        test,
                        load_test,
                        |(state, expected): (spec::BeaconState, RewardsDeltas<{spec::VALIDATOR_REGISTRY_LIMIT}>), context| {
                            run_test(&state, context, expected, |state, context| {
                                let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                                let source_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                                let target_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                                let head_deltas = spec::get_flag_index_deltas(&state, flag_index, context).unwrap();
                                let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(&state, context).unwrap();
                                (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                            })
                        }
                    }
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
