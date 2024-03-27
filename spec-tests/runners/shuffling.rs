use crate::{
    runners::{gen_exec, gen_match_for},
    test_case::TestCase,
    test_utils::{load_yaml, Error},
};
use ethereum_consensus::primitives::Bytes32;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ShufflingTestData {
    seed: Bytes32,
    count: usize,
    mapping: Vec<usize>,
}

fn load_test(test_case_path: &str) -> ShufflingTestData {
    let path = test_case_path.to_string() + "/mapping.yaml";
    load_yaml(&path)
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "core" => {
            gen_match_for! {
                test,
                (mainnet, phase0),
                (minimal, phase0)
                {
                    gen_exec! {
                        test,
                        load_test,
                        |data: ShufflingTestData, context| {
                            // test `compute_shuffled_index`, following the spec which goes index by index
                            let result = (0..data.count).into_iter().map(|index| {
                                spec::compute_shuffled_index(index, data.count, &data.seed, context).unwrap()
                            }).collect::<Vec<_>>();
                            assert_eq!(result, data.mapping);

                            // test `compute_shuffled_indices`, an optimization that shuffles the entire list at once
                            let indices = (0..data.count).collect::<Vec<_>>();
                            let shuffled_indices = spec::compute_shuffled_indices(&indices, &data.seed, context);
                            assert_eq!(shuffled_indices, data.mapping);
                            Ok(())
                        }
                    }
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
