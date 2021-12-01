#![cfg(feature = "spec_tests")]

use ethereum_consensus::crypto::{
    aggregate, aggregate_verify, fast_aggregate_verify, PublicKey, SecretKey, Signature,
};
use glob::glob;
use hex;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_yaml;
use std::fmt::Debug;
use std::fs::File;

fn decode_hex_with_prefix<'a, D>(input: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'a>,
{
    let data: String = Deserialize::deserialize(input)?;
    let data: &[u8] = &data.as_bytes();
    let data = if data.starts_with(b"0x") {
        &data[2..]
    } else {
        data
    };
    //null character case, empty array
    if data == [126] {
        return Ok(vec![]);
    }
    hex::decode(data).map_err(D::Error::custom)
}
fn decode_hexvec_with_prefix<'a, D>(input: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'a>,
{
    let data: Vec<String> = Deserialize::deserialize(input)?;
    println!("{:?}", data);
    if data.is_empty() {
        println!("empty");
        return Ok(vec![]);
    }
    let data: Vec<Vec<u8>> = data
        .iter()
        .map(|x| {
            let x = x.as_bytes();
            if x.starts_with(b"0x") {
                hex::decode(&x[2..]).map_err(D::Error::custom).unwrap()
            } else {
                if x == [126] {
                    vec![]
                } else {
                    hex::decode(x).map_err(D::Error::custom).unwrap()
                }
            }
        })
        .collect();
    Ok(data)
}

#[derive(Debug, Serialize, Deserialize)]
struct SigningInput {
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    privkey: Vec<u8>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    message: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SigningTestIO {
    input: SigningInput,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    output: Vec<u8>,
}

impl TestDriver<SigningTestIO> for SigningTestIO {
    fn verify(&self) {
        let secret_key = match SecretKey::from_bytes(&self.input.privkey) {
            Ok(sk) => sk,
            // this is the empty secret key case
            Err(_) => {
                assert!(self.output.is_empty());
                return;
            }
        };
        let signature = secret_key.sign(&self.input.message);
        let expected_signature = Signature::from_bytes(&self.output).unwrap();
        assert_eq!(signature, expected_signature)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AggregatingTestIO {
    #[serde(deserialize_with = "decode_hexvec_with_prefix")]
    input: Vec<Vec<u8>>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    output: Vec<u8>,
}

impl TestDriver<AggregatingTestIO> for AggregatingTestIO {
    fn verify(&self) {
        let input_signatures: Vec<Signature> = self
            .input
            .iter()
            .map(|x| Signature::from_bytes(x).unwrap())
            .collect();
        let aggregate = match aggregate(&input_signatures) {
            Ok(agg) => agg,
            // handling for zero sized input and output
            Err(_) => {
                assert!(self.output.is_empty());
                return;
            }
        };
        let expected_aggregate = Signature::from_bytes(&self.output).unwrap();
        assert_eq!(aggregate, expected_aggregate)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AggVerifyInput {
    #[serde(deserialize_with = "decode_hexvec_with_prefix")]
    pubkeys: Vec<Vec<u8>>,
    #[serde(deserialize_with = "decode_hexvec_with_prefix")]
    messages: Vec<Vec<u8>>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AggVerifyTestIO {
    input: AggVerifyInput,
    output: bool,
}

impl TestDriver<AggVerifyTestIO> for AggVerifyTestIO {
    fn verify(&self) {
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

#[derive(Debug, Serialize, Deserialize)]
struct FastAggVerifyInput {
    #[serde(deserialize_with = "decode_hexvec_with_prefix")]
    pubkeys: Vec<Vec<u8>>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    message: Vec<u8>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FastAggVerifyTestIO {
    input: FastAggVerifyInput,
    output: bool,
}

impl TestDriver<FastAggVerifyTestIO> for FastAggVerifyTestIO {
    fn verify(&self) {
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
        let pubkeys_ref: Vec<&PublicKey> = pubkeys.iter().map(|x| x).collect();
        let signature = match Signature::from_bytes(&self.input.signature) {
            Ok(sk) => sk,
            // error handling for zero signature
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let verify_result =
            fast_aggregate_verify(pubkeys_ref.as_slice(), &self.input.message, &signature);
        assert_eq!(verify_result, self.output)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VerifyInput {
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    pubkey: Vec<u8>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    message: Vec<u8>,
    #[serde(deserialize_with = "decode_hex_with_prefix")]
    signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerifyTestIO {
    input: VerifyInput,
    output: bool,
}

impl TestDriver<VerifyTestIO> for VerifyTestIO {
    fn verify(&self) {
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

trait TestDriver<T: TestDriver<T> + for<'de> serde::Deserialize<'de>> {
    fn verify(&self) {
        ()
    }

    fn execute_test_cases(path_glob: &str) {
        let entries = glob(path_glob).expect("Failed to read glob pattern");
        for entry in entries {
            let path = entry.unwrap();
            let file = File::open(path).expect("File does not exist");
            let test_case: T = serde_yaml::from_reader(file).expect("Is not well-formatted yaml");
            test_case.verify()
        }
    }
}

#[test]
fn test_fast_aggregate_verify() {
    FastAggVerifyTestIO::execute_test_cases(
        "consensus-spec-tests/tests/general/phase0/bls/fast_aggregate_verify/small/**/*.yaml",
    )
}

#[test]
fn test_verify() {
    VerifyTestIO::execute_test_cases(
        "consensus-spec-tests/tests/general/phase0/bls/verify/small/**/*.yaml",
    )
}

#[test]
fn test_aggregate_verify() {
    AggVerifyTestIO::execute_test_cases(
        "consensus-spec-tests/tests/general/phase0/bls/aggregate_verify/small/**/*.yaml",
    )
}

#[test]
fn test_aggregate() {
    AggregatingTestIO::execute_test_cases(
        "consensus-spec-tests/tests/general/phase0/bls/aggregate/small/**/*.yaml",
    )
}

#[test]
fn test_sign() {
    SigningTestIO::execute_test_cases(
        "consensus-spec-tests/tests/general/phase0/bls/sign/small/**/*.yaml",
    )
}
