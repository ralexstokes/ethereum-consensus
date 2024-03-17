use crate::{
    test_case::TestCase,
    test_utils::{load_yaml, Error},
};
use ethereum_consensus::{
    crypto::{
        aggregate, aggregate_verify, eth_aggregate_public_keys, eth_fast_aggregate_verify,
        fast_aggregate_verify, verify_signature, PublicKey, SecretKey, Signature,
    },
    primitives::Bytes32,
    serde as eth_serde,
};
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};
use std::fmt;

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    match meta.handler.0.as_str() {
        "eth_aggregate_pubkeys" => {
            let path = &test.data_path;
            let test_case = EthAggregatePubkeysTestCase::from(path);
            test_case.execute();
            Ok(())
        }
        "eth_fast_aggregate_verify" => {
            let path = &test.data_path;
            let test_case = EthFastAggregateVerifyTestCase::from(path);
            test_case.execute();
            Ok(())
        }
        "verify" => {
            let path = &test.data_path;
            let test_case = VerifyTestCase::from(path);
            test_case.execute();
            Ok(())
        }
        "aggregate" => {
            let path = &test.data_path;
            let test_case = AggregateTestCase::from(path);
            test_case.execute();
            Ok(())
        }
        "fast_aggregate_verify" => {
            let path = &test.data_path;
            let test_case = FastAggregateVerifyTestCase::from(path);
            test_case.execute();
            Ok(())
        }
        "aggregate_verify" => {
            let path = &test.data_path;
            let test_case = AggregateVerifyTestCase::from(path);
            test_case.execute();
            Ok(())
        }
        "sign" => {
            let path = &test.data_path;
            let test_case = SignTestCase::from(path);
            test_case.execute();
            Ok(())
        }
        handler => Err(Error::UnknownHandler(handler.into(), meta.name())),
    }
}

pub trait Execute: fmt::Debug {
    fn should_succeed(&self) -> bool;

    fn verify_success(&self) -> bool;

    fn verify_failure(&self) -> bool;

    fn execute(&self) {
        let result =
            if self.should_succeed() { self.verify_success() } else { self.verify_failure() };
        assert!(result, "{self:#?}")
    }
}

#[derive(Debug, Deserialize)]
pub struct AggregateTestCase {
    input: Vec<Signature>,
    output: Option<Signature>,
}

impl AggregateTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }
}

impl Execute for AggregateTestCase {
    fn should_succeed(&self) -> bool {
        self.output.is_some()
    }

    fn verify_success(&self) -> bool {
        let aggregation = aggregate(&self.input).unwrap();
        self.output.as_ref().unwrap() == &aggregation
    }

    fn verify_failure(&self) -> bool {
        let aggregation = aggregate(&self.input);
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
pub struct AggregateVerifyTestCase {
    input: AggregateVerifyInput,
    output: bool,
}

impl AggregateVerifyTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }

    pub fn run(&self) -> bool {
        let public_keys = self.input.pubkeys.iter().flatten().cloned().collect::<Vec<_>>();
        let messages = self.input.messages.iter().map(|m| m.as_ref()).collect::<Vec<_>>();
        let signature = self.input.signature.as_ref().unwrap();
        aggregate_verify(&public_keys, &messages, signature).is_ok()
    }
}

impl Execute for AggregateVerifyTestCase {
    fn should_succeed(&self) -> bool {
        self.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.input.signature.is_none() {
            return true
        }
        !self.run()
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct FastAggregateVerifyInput {
    pubkeys: Vec<String>,
    message: Bytes32,
    #[serde_as(deserialize_as = "DefaultOnError")]
    signature: Option<Signature>,
}

#[derive(Debug, Deserialize)]
pub struct FastAggregateVerifyTestCase {
    input: FastAggregateVerifyInput,
    output: bool,
}

impl FastAggregateVerifyTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }

    pub fn run(&self) -> bool {
        let public_keys = &self
            .input
            .pubkeys
            .iter()
            .map(|repr| serde_yaml::from_str(repr).unwrap())
            .collect::<Vec<PublicKey>>();
        let public_keys = public_keys.iter().collect::<Vec<_>>();
        fast_aggregate_verify(
            &public_keys,
            self.input.message.as_ref(),
            self.input.signature.as_ref().unwrap(),
        )
        .is_ok()
    }
}

impl Execute for FastAggregateVerifyTestCase {
    fn should_succeed(&self) -> bool {
        self.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.input.signature.is_none() {
            return true
        }
        if self.input.pubkeys.iter().any(|key| {
            let input: Result<PublicKey, _> = serde_yaml::from_str(key);
            input.is_err()
        }) {
            return true
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
pub struct SignTestCase {
    input: SignInput,
    output: Option<Signature>,
}

impl SignTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }
}

impl Execute for SignTestCase {
    fn should_succeed(&self) -> bool {
        self.output.is_some()
    }

    fn verify_success(&self) -> bool {
        let signature = self.input.privkey.as_ref().unwrap().sign(self.input.message.as_ref());
        &signature == self.output.as_ref().unwrap()
    }

    fn verify_failure(&self) -> bool {
        self.input.privkey.is_none()
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct VerifyInput {
    #[serde_as(deserialize_as = "DefaultOnError")]
    pubkey: Option<PublicKey>,
    message: Bytes32,
    #[serde_as(deserialize_as = "DefaultOnError")]
    signature: Option<Signature>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyTestCase {
    input: VerifyInput,
    output: bool,
}

impl VerifyTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }

    fn run(&self) -> bool {
        verify_signature(
            self.input.pubkey.as_ref().unwrap(),
            self.input.message.as_ref(),
            self.input.signature.as_ref().unwrap(),
        )
        .is_ok()
    }
}

impl Execute for VerifyTestCase {
    fn should_succeed(&self) -> bool {
        self.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.input.signature.is_none() {
            return true
        }
        if self.input.pubkey.is_none() {
            return true
        }
        !self.run()
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct EthAggregatePubkeysTestCase {
    #[serde_as(deserialize_as = "DefaultOnError")]
    input: Vec<PublicKey>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    output: Option<PublicKey>,
}

impl EthAggregatePubkeysTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }

    pub fn run(&self) -> bool {
        let aggregate_public_key = eth_aggregate_public_keys(&self.input).unwrap();
        &aggregate_public_key == self.output.as_ref().unwrap()
    }
}

impl Execute for EthAggregatePubkeysTestCase {
    fn should_succeed(&self) -> bool {
        self.output.is_some()
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        self.output.is_none()
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct EthFastAggregateVerifyInput {
    #[serde(rename = "pubkeys")]
    public_keys: Vec<String>,
    message: Bytes32,
    #[serde_as(deserialize_as = "DefaultOnError")]
    signature: Option<Signature>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct EthFastAggregateVerifyTestCase {
    input: EthFastAggregateVerifyInput,
    output: bool,
}

impl EthFastAggregateVerifyTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }

    pub fn run(&self) -> bool {
        let public_keys = &self
            .input
            .public_keys
            .iter()
            .map(|repr| serde_yaml::from_str(repr).unwrap())
            .collect::<Vec<PublicKey>>();
        let public_keys = public_keys.iter().collect::<Vec<_>>();
        let message = self.input.message.as_ref();
        let signature = self.input.signature.as_ref().unwrap();
        eth_fast_aggregate_verify(&public_keys, message, signature).is_ok()
    }
}

impl Execute for EthFastAggregateVerifyTestCase {
    fn should_succeed(&self) -> bool {
        self.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.input.signature.is_none() {
            return true
        }

        if self.input.public_keys.iter().any(|key| {
            let input: Result<PublicKey, _> = serde_yaml::from_str(key);
            input.is_err()
        }) {
            return true
        }

        !self.run()
    }
}
