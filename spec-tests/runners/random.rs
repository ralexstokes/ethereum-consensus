use crate::{
    runners::{gen_dispatch, gen_exec},
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

macro_rules! run_test {
    ($context:expr, $pre:ident, $post: ident, $blocks:ident) => {
        let mut pre = $pre;
        let mut blocks = $blocks;
        for block in blocks.iter_mut() {
            todo!(/*
                TODO: move exec engine as member of `Context`
                spec::state_transition(&mut pre, block, Validation::Enabled, $context).map_err(Error::from)?;
                 */);
        }
        let result = Ok::<_, Error>(());
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
        "random" => {
            gen_dispatch! {
                path,
                meta.config,
                meta.fork,
                |path| { load_test::<spec::BeaconState, spec::SignedBeaconBlock>(path) },
                |(pre, post, blocks): (spec::BeaconState, Option<spec::BeaconState>, Vec<spec::SignedBeaconBlock>), context| {
                    run_test! { context, pre, post, blocks }
                }
            }
        }
        handler => Err(Error::UnknownHandler(handler.into(), meta.name())),
    }
}
