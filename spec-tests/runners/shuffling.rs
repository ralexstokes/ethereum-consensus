use crate::{
    runners::{gen_dispatch, gen_exec},
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
    let meta = &test.meta;
    let path = &test.data_path;
    match meta.handler.0.as_str() {
        "core" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                load_test,
                |data: ShufflingTestData, context| {
                    for index in 0..data.count {
                        let result = spec::compute_shuffled_index(index, data.count, &data.seed, context).unwrap();
                        assert_eq!(result, data.mapping[index]);
                    }
                    Ok(())
                }
            }
        }
        handler => Err(Error::UnknownHandler(handler.into(), meta.name())),
    }
}
