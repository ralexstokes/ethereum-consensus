#![cfg(feature = "spec-tests")]

mod test_utils;

use ethereum_consensus::crypto::{
    aggregate, aggregate_verify, fast_aggregate_verify, PublicKey, SecretKey, Signature,
};
use serde::Deserialize;
use test_utils::{
    decode_hex_vec_with_prefix, decode_hex_with_prefix, decode_hex_with_prefix_maybe, TestCase,
};

#[derive(Debug, Deserialize)]
struct SigningInput {
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    privkey: Vec<u8>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    message: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct SigningTestIO {
    input: SigningInput,
    #[serde(deserialize_with = "decode_hex_with_prefix_maybe")]
    output: Option<Vec<u8>>,
}

impl TestCase for SigningTestIO {
    fn execute(&self) {
        let secret_key = match SecretKey::from_bytes(&self.input.privkey) {
            Ok(sk) => sk,
            // this is the empty secret key case
            Err(_) => {
                assert!(self.output.is_none());
                return;
            }
        };
        let signature = secret_key.sign(&self.input.message);
        let output_value = self.output.as_ref().unwrap();
        let expected_signature = Signature::from_bytes(output_value).unwrap();
        assert_eq!(signature, expected_signature)
    }
}

#[derive(Debug, Deserialize)]
struct AggregatingTestIO {
    #[serde(deserialize_with = "decode_hex_vec_with_prefix")]
    input: Vec<Vec<u8>>,
    #[serde(deserialize_with = "decode_hex_with_prefix_maybe")]
    output: Option<Vec<u8>>,
}

impl TestCase for AggregatingTestIO {
    fn execute(&self) {
        let input_signatures: Vec<Signature> = self
            .input
            .iter()
            .map(|x| Signature::from_bytes(x).unwrap())
            .collect();
        let aggregate = match aggregate(&input_signatures) {
            Ok(agg) => agg,
            // handling for zero sized input and output
            Err(_) => {
                assert!(self.output.is_none());
                return;
            }
        };
        let output_value = self.output.as_ref().unwrap();
        let expected_aggregate = Signature::from_bytes(output_value).unwrap();
        assert_eq!(aggregate, expected_aggregate)
    }
}

#[derive(Debug, Deserialize)]
struct AggVerifyInput {
    #[serde(deserialize_with = "decode_hex_vec_with_prefix")]
    pubkeys: Vec<Vec<u8>>,
    #[serde(deserialize_with = "decode_hex_vec_with_prefix")]
    messages: Vec<Vec<u8>>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    signature: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct AggVerifyTestIO {
    input: AggVerifyInput,
    output: bool,
}

impl TestCase for AggVerifyTestIO {
    fn execute(&self) {
        let pubkeys_result: Result<Vec<PublicKey>, _> = self
            .input
            .pubkeys
            .iter()
            .map(|x| PublicKey::from_bytes(&x))
            .collect();
        let pubkeys = match pubkeys_result {
            Ok(pk) => pk,
            // error handling for infinity pub key
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let messages: Vec<&[u8]> = self.input.messages.iter().map(|x| x.as_slice()).collect();
        let signature = match Signature::from_bytes(&self.input.signature) {
            Ok(sign) => sign,
            // handling for the zero signature case which raises a blst bad encoding error
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let verify_result = aggregate_verify(&pubkeys, &messages, signature);
        assert_eq!(verify_result, self.output)
    }
}

#[derive(Debug, Deserialize)]
struct FastAggVerifyInput {
    #[serde(deserialize_with = "decode_hex_vec_with_prefix")]
    pubkeys: Vec<Vec<u8>>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    message: Vec<u8>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    signature: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct FastAggVerifyTestIO {
    input: FastAggVerifyInput,
    output: bool,
}

impl TestCase for FastAggVerifyTestIO {
    fn execute(&self) {
        let pubkeys_result: Result<Vec<PublicKey>, _> = self
            .input
            .pubkeys
            .iter()
            .map(|x| PublicKey::from_bytes(x))
            .collect();
        let pubkeys = match pubkeys_result {
            Ok(pk) => pk,
            // error handling for infinity pub key
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let pubkeys: Vec<&PublicKey> = pubkeys.iter().collect();
        let signature = match Signature::from_bytes(&self.input.signature) {
            Ok(sk) => sk,
            // error handling for zero signature
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let verify_result = fast_aggregate_verify(&pubkeys, &self.input.message, &signature);
        assert_eq!(verify_result, self.output)
    }
}

#[derive(Debug, Deserialize)]
struct VerifyInput {
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    pubkey: Vec<u8>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    message: Vec<u8>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    signature: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct VerifyTestIO {
    input: VerifyInput,
    output: bool,
}

impl TestCase for VerifyTestIO {
    fn execute(&self) {
        let pubkey: PublicKey = match PublicKey::from_bytes(&self.input.pubkey) {
            Ok(pk) => pk,
            // error handling for infinity pub key
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let signature = match Signature::from_bytes(&self.input.signature) {
            Ok(sk) => sk,
            // this is the case where the tampered signature cannot be read in as real signature
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let verify_result = signature.verify(&pubkey, &self.input.message);
        assert_eq!(verify_result, self.output)
    }
}

#[test]
fn test_fast_aggregate_verify() {
    FastAggVerifyTestIO::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/fast_aggregate_verify/small/**/*.yaml",
    )
}

#[test]
fn test_verify() {
    VerifyTestIO::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/verify/small/**/*.yaml",
    )
}

#[test]
fn test_aggregate_verify() {
    AggVerifyTestIO::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/aggregate_verify/small/**/*.yaml",
    )
}

#[test]
fn test_aggregate() {
    AggregatingTestIO::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/aggregate/small/**/*.yaml",
    )
}

#[test]
fn test_sign() {
    SigningTestIO::execute_from_glob(
        "consensus-spec-tests/tests/general/phase0/bls/sign/small/**/*.yaml",
    )
}
