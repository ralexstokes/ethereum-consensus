use crate::test_utils::{load_yaml, TestCase};
use ethereum_consensus::crypto::{
    aggregate, aggregate_verify, eth_aggregate_public_keys, fast_aggregate_verify, PublicKey,
    SecretKey, Signature,
};
use ethereum_consensus::primitives::Bytes32;
use ethereum_consensus::serde as eth_serde;
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};

#[derive(Debug, Deserialize)]
struct AggregateTestCase {
    input: Vec<Signature>,
    output: Option<Signature>,
}

#[derive(Debug)]
pub struct AggregateHandler {
    test_case: AggregateTestCase,
}

impl AggregateHandler {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        let test_case: AggregateTestCase = load_yaml(&path);
        Self { test_case }
    }
}

impl TestCase for AggregateHandler {
    fn should_succeed(&self) -> bool {
        self.test_case.output.is_some()
    }

    fn verify_success(&self) -> bool {
        let aggregation = aggregate(&self.test_case.input).unwrap();
        self.test_case.output.as_ref().unwrap() == &aggregation
    }

    fn verify_failure(&self) -> bool {
        let aggregation = aggregate(&self.test_case.input);
        aggregation.is_err()
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct AggregateVerifyInput {
    #[serde_as(deserialize_as = "DefaultOnError")]
    pubkeys: Vec<Option<PublicKey>>,
    messages: Vec<Bytes32>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    signature: Option<Signature>,
}

#[derive(Debug, Deserialize)]
struct AggregateVerifyTestCase {
    input: AggregateVerifyInput,
    output: bool,
}

#[derive(Debug)]
pub struct AggregateVerifyHandler {
    test_case: AggregateVerifyTestCase,
}

impl AggregateVerifyHandler {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        let test_case: AggregateVerifyTestCase = load_yaml(&path);
        Self { test_case }
    }

    pub fn run(&self) -> bool {
        let public_keys = self
            .test_case
            .input
            .pubkeys
            .iter()
            .cloned()
            .filter_map(|k| k)
            .collect::<Vec<_>>();
        let messages = self
            .test_case
            .input
            .messages
            .iter()
            .map(|m| m.as_ref())
            .collect::<Vec<_>>();
        let signature = self.test_case.input.signature.as_ref().unwrap();
        aggregate_verify(&public_keys, &messages, signature)
    }
}

impl TestCase for AggregateVerifyHandler {
    fn should_succeed(&self) -> bool {
        self.test_case.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.test_case.input.signature.is_none() {
            return true;
        }
        !self.run()
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct FastAggregateVerifyInput {
    pubkeys: Vec<PublicKey>,
    message: Bytes32,
    #[serde_as(deserialize_as = "DefaultOnError")]
    signature: Option<Signature>,
}

#[derive(Debug, Deserialize)]
struct FastAggregateVerifyTestCase {
    input: FastAggregateVerifyInput,
    output: bool,
}

#[derive(Debug)]
pub struct FastAggregateVerifyHandler {
    test_case: FastAggregateVerifyTestCase,
}

impl FastAggregateVerifyHandler {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        let test_case: FastAggregateVerifyTestCase = load_yaml(&path);
        Self { test_case }
    }

    pub fn run(&self) -> bool {
        let public_keys = self.test_case.input.pubkeys.iter().collect::<Vec<_>>();
        fast_aggregate_verify(
            &public_keys,
            self.test_case.input.message.as_ref(),
            self.test_case.input.signature.as_ref().unwrap(),
        )
    }
}

impl TestCase for FastAggregateVerifyHandler {
    fn should_succeed(&self) -> bool {
        self.test_case.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.test_case.input.signature.is_none() {
            return true;
        }
        if self
            .test_case
            .input
            .pubkeys
            .iter()
            .any(|k| k == &PublicKey::default())
        {
            return true;
        }

        !self.run()
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

#[serde_as]
#[derive(Debug, Deserialize)]
struct VerifyInput {
    pubkey: PublicKey,
    message: Bytes32,
    #[serde_as(deserialize_as = "DefaultOnError")]
    signature: Option<Signature>,
}

#[derive(Debug, Deserialize)]
struct VerifyTestCase {
    input: VerifyInput,
    output: bool,
}

#[derive(Debug)]
pub struct VerifyHandler {
    test_case: VerifyTestCase,
}

impl VerifyHandler {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        let test_case: VerifyTestCase = load_yaml(&path);
        Self { test_case }
    }

    fn run(&self) -> bool {
        self.test_case.input.signature.as_ref().unwrap().verify(
            &self.test_case.input.pubkey,
            self.test_case.input.message.as_ref(),
        )
    }
}

impl TestCase for VerifyHandler {
    fn should_succeed(&self) -> bool {
        self.test_case.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.test_case.input.signature.is_none() {
            return true;
        }
        !self.run()
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct EthAggregatePubkeysTestCase {
    #[serde_as(deserialize_as = "DefaultOnError")]
    input: Vec<PublicKey>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    output: Option<PublicKey>,
}

#[derive(Debug)]
pub struct EthAggregatePubkeysHandler {
    test_case: EthAggregatePubkeysTestCase,
}

impl EthAggregatePubkeysHandler {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        let test_case: EthAggregatePubkeysTestCase = load_yaml(&path);
        Self { test_case }
    }

    pub fn run(&self) -> bool {
        let aggregate_public_key = eth_aggregate_public_keys(&self.test_case.input).unwrap();
        &aggregate_public_key == self.test_case.output.as_ref().unwrap()
    }
}

impl TestCase for EthAggregatePubkeysHandler {
    fn should_succeed(&self) -> bool {
        self.test_case.output.is_some()
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        self.test_case.output.is_none()
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
