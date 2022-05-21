#![cfg(feature = "spec-tests")]

mod test_utils;

use ethereum_consensus::crypto::{
    aggregate, aggregate_verify, fast_aggregate_verify, PublicKey, SecretKey, Signature,
};
use ethereum_consensus::primitives::Bytes32;
use ethereum_consensus::serde as eth_serde;
use serde::Deserialize;
use test_utils::TestCase;

#[derive(Debug, Deserialize)]
struct SignInput {
    #[serde(deserialize_with = "eth_serde::as_hex::deserialize")]
    privkey: Vec<u8>,
    #[serde(deserialize_with = "eth_serde::as_hex::deserialize")]
    message: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct SignTest {
    input: SignInput,
    output: Option<Signature>,
}

impl TestCase for SignTest {
    fn execute(self) {
        match self.output {
            Some(output) => {
                let secret_key = SecretKey::from_bytes(&self.input.privkey).unwrap();
                let signature = secret_key.sign(&self.input.message);
                assert_eq!(signature, output)
            }
            None => assert!(SecretKey::from_bytes(&self.input.privkey).is_err()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AggregateTest {
    input: Vec<Signature>,
    output: Option<Signature>,
}

impl TestCase for AggregateTest {
    fn execute(self) {
        let aggregation = aggregate(&self.input);
        match self.output {
            Some(output) => assert_eq!(output, aggregation.unwrap()),
            None => assert!(aggregation.is_err()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AggregateVerifyInput {
    pubkeys: Vec<String>,
    messages: Vec<Bytes32>,
    signature: String,
}

#[derive(Debug, Deserialize)]
struct AggregateVerifyTest {
    input: AggregateVerifyInput,
    output: bool,
}

impl TestCase for AggregateVerifyTest {
    fn execute(self) {
        let signature = self.input.signature.try_into();
        if self.output {
            let public_keys = self
                .input
                .pubkeys
                .into_iter()
                .map(|key| PublicKey::try_from(key).unwrap())
                .collect::<Vec<_>>();
            let messages = self
                .input
                .messages
                .iter()
                .map(|m| m.as_ref())
                .collect::<Vec<_>>();
            let aggregation = aggregate_verify(&public_keys, &messages, signature.unwrap());
            assert!(aggregation);
        } else {
            assert!(signature.is_err())
        }
    }
}

#[derive(Debug, Deserialize)]
struct VerifyInput {
    pubkey: String,
    message: Bytes32,
    signature: Signature,
}

#[derive(Debug, Deserialize)]
struct VerifyTest {
    input: VerifyInput,
    output: bool,
}

impl TestCase for VerifyTest {
    fn execute(self) {
        let success = PublicKey::try_from(self.input.pubkey).map(|public_key| {
            self.input
                .signature
                .verify(&public_key, self.input.message.as_ref())
        });
        if self.output {
            assert!(success.unwrap());
        } else {
            assert!(success.is_err());
        }
    }
}

#[derive(Debug, Deserialize)]
struct FastAggregateVerifyInput {
    pubkeys: Vec<PublicKey>,
    message: Bytes32,
    signature: Signature,
}

#[derive(Debug, Deserialize)]
struct FastAggregateVerifyTest {
    input: FastAggregateVerifyInput,
    output: bool,
}

impl TestCase for FastAggregateVerifyTest {
    fn execute(self) {
        let public_keys = self.input.pubkeys.iter().collect::<Vec<_>>();
        let success = fast_aggregate_verify(
            &public_keys,
            self.input.message.as_ref(),
            &self.input.signature,
        );
        assert_eq!(success, self.output);
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
