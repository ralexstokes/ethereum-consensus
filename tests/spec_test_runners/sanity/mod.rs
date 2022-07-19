use crate::test_utils::{load_snappy_ssz, load_yaml, Config, TestCase};
use ethereum_consensus::primitives::Slot;
use ethereum_consensus::serde as eth_serde;
use ethereum_consensus::state_transition::Context;
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};
use std::fmt;

pub struct BlocksTestCase;

impl BlocksTestCase {
    pub fn from(test_case_path: &str) -> Self {
        Self
    }

    pub fn execute(&self) {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct SlotsTestCase<S> {
    pre: S,
    post: S,
    slots: Slot,
    config: Config,
}

impl<S> SlotsTestCase<S>
where
    S: fmt::Debug + ssz_rs::Deserialize + for<'de> serde::Deserialize<'de> + PartialEq<S>,
{
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/pre.ssz_snappy";
        let pre: S = load_snappy_ssz(&path);

        let path = test_case_path.to_string() + "/post.ssz_snappy";
        let post: S = load_snappy_ssz(&path);

        let path = test_case_path.to_string() + "/slots.yaml";
        let slots: Slot = load_yaml(&path);

        let config = if test_case_path.contains("minimal") {
            Config::Minimal
        } else {
            Config::Mainnet
        };

        Self {
            pre,
            post,
            slots,
            config,
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: Fn(&mut S, Slot, &Context),
    {
        let context = match self.config {
            Config::Minimal => Context::for_minimal(),
            Config::Mainnet => Context::for_mainnet(),
        };
        f(&mut self.pre, self.slots, &context);
        assert_eq!(self.pre, self.post);
    }
}
