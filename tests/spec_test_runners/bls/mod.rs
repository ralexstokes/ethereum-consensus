use crate::test_utils::{load_yaml, TestCase};
use ethereum_consensus::crypto::{
    // aggregate, aggregate_verify, fast_aggregate_verify, PublicKey,
    SecretKey,
    Signature,
};
// use ethereum_consensus::primitives::Bytes32;
use ethereum_consensus::serde as eth_serde;
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};

pub struct AggregateHandler;

impl AggregateHandler {
    pub fn from(test_case_path: &str) -> Self {
        Self
    }

    pub fn execute(&self) {
        unimplemented!();
    }
}

pub struct AggregateVerifyHandler;

impl AggregateVerifyHandler {
    pub fn from(test_case_path: &str) -> Self {
        Self
    }

    pub fn execute(&self) {
        unimplemented!();
    }
}

pub struct FastAggregateVerifyHandler;

impl FastAggregateVerifyHandler {
    pub fn from(test_case_path: &str) -> Self {
        Self
    }

    pub fn execute(&self) {
        unimplemented!();
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct SignInput {
    #[serde_as(deserialize_as = "DefaultOnError")]
    privkey: Option<SecretKey>,
    #[serde(deserialize_with = "eth_serde::as_hex::deserialize")]
    message: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct SignTestCase {
    input: SignInput,
    output: Option<Signature>,
}

#[derive(Debug)]
pub struct SignHandler {
    test_case: SignTestCase,
}

impl SignHandler {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        let test_case: SignTestCase = load_yaml(&path);
        Self { test_case }
    }
}

impl TestCase for SignHandler {
    fn should_succeed(&self) -> bool {
        self.test_case.output.is_some()
    }

    fn verify_success(&self) -> bool {
        let signature = self
            .test_case
            .input
            .privkey
            .as_ref()
            .unwrap()
            .sign(self.test_case.input.message.as_ref());
        &signature == self.test_case.output.as_ref().unwrap()
    }

    fn verify_failure(&self) -> bool {
        self.test_case.input.privkey.is_none()
    }
}

pub struct VerifyHandler;

impl VerifyHandler {
    pub fn from(test_case_path: &str) -> Self {
        Self
    }

    pub fn execute(&self) {
        unimplemented!();
    }
}

pub struct EthAggregatePubkeysHandler;

impl EthAggregatePubkeysHandler {
    pub fn from(test_case_path: &str) -> Self {
        Self
    }

    pub fn execute(&self) {
        unimplemented!();
    }
}

pub struct EthFastAggregateVerifyHandler;

impl EthFastAggregateVerifyHandler {
    pub fn from(test_case_path: &str) -> Self {
        Self
    }

    pub fn execute(&self) {
        unimplemented!();
    }
}
