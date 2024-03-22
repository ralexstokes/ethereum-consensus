use crate::{
    runners::{gen_exec, gen_match_for_all},
    test_case::TestCase,
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct RandomMeta {
    blocks_count: usize,
}

fn load_test<S: ssz_rs::Deserialize, B: ssz_rs::Deserialize>(
    test_case_path: &str,
) -> (S, Option<S>, Vec<B>) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: Option<S> = load_snappy_ssz(&path);

    let path = test_case_path.to_string() + "/meta.yaml";
    let meta: RandomMeta = load_yaml(&path);
    let blocks_count = meta.blocks_count;

    let mut blocks = vec![];
    for i in 0..blocks_count {
        let path = format!("{test_case_path}/blocks_{i}.ssz_snappy");
        let block: B = load_snappy_ssz(&path).unwrap();
        blocks.push(block);
    }

    (pre, post, blocks)
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "random" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post, blocks): (spec::BeaconState, Option<spec::BeaconState>, Vec<spec::SignedBeaconBlock>), _context| {
                    crate::runners::sanity::run_test(pre, post, blocks, _context)
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
