use crate::test_utils::{load_snappy_ssz, load_yaml, Config};
use ethereum_consensus::primitives::Epoch;
use ethereum_consensus::state_transition::{Context, Result};
use serde::Deserialize;
use std::fmt;

pub struct CoreTestCase<S> {
    test_case_path: String,
    post: S,
    meta: Meta,
    config: Config,
}

#[derive(Deserialize)]
struct Meta {
    post_fork: String,
    fork_epoch: Epoch,
    blocks_count: usize,
}

impl<S> CoreTestCase<S>
where
    S: fmt::Debug + ssz_rs::Deserialize + PartialEq<S>,
{
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/post.ssz_snappy";
        let post: S = load_snappy_ssz(&path).unwrap();

        let path = test_case_path.to_string() + "/meta.yaml";
        let meta: Meta = load_yaml(&path);

        let config = if test_case_path.contains("minimal") {
            Config::Minimal
        } else {
            Config::Mainnet
        };

        Self {
            test_case_path: test_case_path.to_string(),
            post,
            meta,
            config,
        }
    }

    pub fn execute<F, B>(&mut self, f: F)
    where
        F: FnOnce(S, Vec<B>, Context) -> Result<S>,
        B: fmt::Debug + ssz_rs::Deserialize + PartialEq<B>,
    {
        let path = self.test_case_path.to_string() + "/pre.ssz_snappy";
        let pre: S = load_snappy_ssz(&path).unwrap();

        let blocks_count = self.meta.blocks_count;
        let mut blocks = vec![];
        for i in 0..blocks_count {
            let path = format!(
                "{}/blocks_{}.ssz_snappy",
                self.test_case_path.to_string(),
                i
            );
            let block: B = load_snappy_ssz(&path).unwrap();
            blocks.push(block);
        }

        let mut context = match self.config {
            Config::Minimal => Context::for_minimal(),
            Config::Mainnet => Context::for_mainnet(),
        };
        match self.meta.post_fork.as_ref() {
            "altair" => {
                context.fork_schedule.altair = self.meta.fork_epoch;
                context.fork_schedule.bellatrix = Epoch::MAX;
            }
            "bellatrix" => {
                context.fork_schedule.altair = 0;
                context.fork_schedule.bellatrix = self.meta.fork_epoch;
            }
            _ => todo!("support other forks"),
        }

        let post = f(pre, blocks, context).unwrap();
        assert_eq!(&post, &self.post);
    }
}
