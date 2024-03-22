use crate::{
    runners::{gen_exec, gen_match_for_all},
    test_case::TestCase,
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use ethereum_consensus::state_transition::Context;
use serde::Deserialize;

#[derive(Deserialize)]
struct BlocksMeta {
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
    let meta: BlocksMeta = load_yaml(&path);
    let blocks_count = meta.blocks_count;

    let mut blocks = vec![];
    for i in 0..blocks_count {
        let path = format!("{test_case_path}/blocks_{i}.ssz_snappy");
        let block: B = load_snappy_ssz(&path).unwrap();
        blocks.push(block);
    }

    (pre, post, blocks)
}

pub(crate) fn run_test<S: Eq, B>(
    pre: S,
    post: Option<S>,
    mut blocks: Vec<B>,
    _context: &Context,
) -> Result<(), Error> {
    for _block in blocks.iter_mut() {
        todo!(/*
                TODO: move exec engine as member of `Context`
                spec::state_transition(&mut pre, block, Validation::Enabled, $context).map_err(Error::from)?;
                 */);
    }
    let result = Result::<(), _>::Err(Error::ExpectedError);
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

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "sanity" => {
            gen_match_for_all! {
                test,
                load_test,
                |(pre, post, blocks): (spec::BeaconState, Option<spec::BeaconState>, Vec<spec::SignedBeaconBlock>), _context| {
                    run_test(pre, post, blocks, _context)
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
