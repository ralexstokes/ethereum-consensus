#![cfg(feature = "spec-tests")]

mod test_utils;

use ethereum_consensus::crypto::{
    aggregate, aggregate_verify, fast_aggregate_verify, PublicKey, SecretKey, Signature,
};
use ethereum_consensus::primitives::Bytes32;
use ethereum_consensus::serde as eth_serde;
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};
use test_utils::TestCase;

#[serde_as]
#[derive(Debug, Deserialize)]
struct SignInput {
    #[serde_as(deserialize_as = "DefaultOnError")]
    privkey: Option<SecretKey>,
    #[serde(deserialize_with = "eth_serde::as_hex::deserialize")]
    message: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct SignTest {
    input: SignInput,
    output: Option<Signature>,
}

impl TestCase for SignTest {
    fn should_succeed(&self) -> bool {
        self.output.is_some()
    }

    fn verify_success(&self) -> bool {
        let signature = self
            .input
            .privkey
            .as_ref()
            .unwrap()
            .sign(self.input.message.as_ref());
        &signature == self.output.as_ref().unwrap()
    }

    fn verify_failure(&self) -> bool {
        self.input.privkey.is_none()
    }
}

#[derive(Debug, Deserialize)]
struct AggregateTest {
    input: Vec<Signature>,
    output: Option<Signature>,
}

impl TestCase for AggregateTest {
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
struct AggregateVerifyTest {
    input: AggregateVerifyInput,
    output: bool,
}

impl AggregateVerifyTest {
    fn run(&self) -> bool {
        let public_keys = self
            .input
            .pubkeys
            .iter()
            .cloned()
            .filter_map(|k| k)
            .collect::<Vec<_>>();
        let messages = self
            .input
            .messages
            .iter()
            .map(|m| m.as_ref())
            .collect::<Vec<_>>();
        let signature = self.input.signature.as_ref().unwrap();
        aggregate_verify(&public_keys, &messages, signature)
    }
}

impl TestCase for AggregateVerifyTest {
    fn should_succeed(&self) -> bool {
        self.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.input.signature.is_none() {
            return true;
        }
        !self.run()
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
struct VerifyTest {
    input: VerifyInput,
    output: bool,
}

impl VerifyTest {
    fn run(&self) -> bool {
        self.input
            .signature
            .as_ref()
            .unwrap()
            .verify(&self.input.pubkey, self.input.message.as_ref())
    }
}

impl TestCase for VerifyTest {
    fn should_succeed(&self) -> bool {
        self.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.input.signature.is_none() {
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
struct FastAggregateVerifyTest {
    input: FastAggregateVerifyInput,
    output: bool,
}

impl FastAggregateVerifyTest {
    fn run(&self) -> bool {
        let public_keys = self.input.pubkeys.iter().collect::<Vec<_>>();
        fast_aggregate_verify(
            &public_keys,
            self.input.message.as_ref(),
            self.input.signature.as_ref().unwrap(),
        )
    }
}

impl TestCase for FastAggregateVerifyTest {
    fn should_succeed(&self) -> bool {
        self.output
    }

    fn verify_success(&self) -> bool {
        self.run()
    }

    fn verify_failure(&self) -> bool {
        if self.input.signature.is_none() {
            return true;
        }
        if self
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

#[test]
fn test_sign() {
    SignTest::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/sign/small/**/*.yaml",
    )
}

#[test]
fn test_aggregate() {
    AggregateTest::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/aggregate/small/**/*.yaml",
    )
}

#[test]
fn test_aggregate_verify() {
    AggregateVerifyTest::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/aggregate_verify/small/**/*.yaml",
    )
}

#[test]
fn test_verify() {
    VerifyTest::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/verify/small/**/*.yaml",
    )
}

#[test]
fn test_fast_aggregate_verify() {
    FastAggregateVerifyTest::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/fast_aggregate_verify/small/**/*.yaml",
    )
}
