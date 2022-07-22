use crate::test_utils::{load_snappy_ssz, load_yaml, Config};
use ethereum_consensus::state_transition::Context;
use serde::Deserialize;

pub struct InitializationTestCase;

impl InitializationTestCase {
    pub fn from(test_case_path: &str) -> Self {
        Self
    }

    pub fn execute(&self) {
        unimplemented!();
    }
}

pub struct ValidityTestCase<S> {
    state: S,
    is_valid: bool,
    config: Config,
}

impl<S> ValidityTestCase<S>
where
    S: ssz_rs::Deserialize,
{
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/genesis.ssz_snappy";
        let state: S = load_snappy_ssz(&path).unwrap();

        let path = test_case_path.to_string() + "/is_valid.yaml";
        let is_valid: bool = load_yaml(&path);

        let config = if test_case_path.contains("minimal") {
            Config::Minimal
        } else {
            todo!("support preset");
        };

        Self {
            state,
            is_valid,
            config,
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce(&S, &Context) -> bool,
    {
        let context = match self.config {
            Config::Minimal => Context::for_minimal(),
            Config::Mainnet => Context::for_mainnet(),
        };
        let is_valid = f(&self.state, &context);
        assert_eq!(is_valid, self.is_valid);
    }
}
