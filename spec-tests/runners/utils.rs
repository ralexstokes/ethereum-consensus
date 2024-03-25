use crate::test_utils::{load_snappy_ssz, load_yaml, Error};
use ethereum_consensus::{state_transition::Context, Error as SpecError};
use serde::Deserialize;

#[derive(Deserialize)]
struct BlocksMeta {
    blocks_count: usize,
}

pub(crate) fn load_blocks_test<S: ssz_rs::Deserialize, B: ssz_rs::Deserialize>(
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

pub(crate) fn run_blocks_test<S: Eq, B, F>(
    mut pre: S,
    post: Option<S>,
    mut blocks: Vec<B>,
    context: &Context,
    exec_fn: F,
) -> Result<(), Error>
where
    F: Fn(&mut S, &mut B, &Context) -> Result<(), SpecError>,
{
    let result = blocks
        .iter_mut()
        .map(|block| exec_fn(&mut pre, block, context))
        .collect::<Result<Vec<_>, SpecError>>();
    if let Some(post) = post {
        assert!(result.is_ok());
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
