#![cfg(feature = "spec_tests")]
use ethereum_consensus::crypto::{
    aggregate, aggregate_verify, fast_aggregate_verify, PublicKey, SecretKey, Signature,
};
use glob::glob;
use hex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fmt::Debug;
use std::fs::File;
fn decode_hex_with_prefix<T: AsRef<[u8]> + Debug>(data: T) -> Vec<u8> {
    let data = data.as_ref();
    let data = if data.starts_with(b"0x") {
        &data[2..]
    } else {
        data
    };
    hex::decode(data).expect("is not well-formed hex")
}
#[derive(Debug, Serialize, Deserialize)]
struct SigningInput {
    privkey: String,
    message: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct SigningTestIO {
    input: SigningInput,
    output: String,
}
impl SigningTestIO {
    fn verify(&self) {
        // let input = &self.input;
        let secret_key_bytes = decode_hex_with_prefix(&self.input.privkey);
        let secret_key = match SecretKey::from_bytes(&secret_key_bytes) {
            Ok(sk) => sk,
            // this is the empty secret key case
            Err(_) => {
                assert_eq!(self.output, "~");
                return;
            }
        };
        let message_bytes = decode_hex_with_prefix(&self.input.message);
        let signature = secret_key.sign(&message_bytes);
        let signature_bytes = decode_hex_with_prefix(&self.output);
        let expected_signature = Signature::from_bytes(&signature_bytes).unwrap();
        assert_eq!(signature, expected_signature)
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct AggregatingTestIO {
    input: Vec<String>,
    output: String,
}
impl AggregatingTestIO {
    fn verify(&self) {
        let input_signatures: Vec<Signature> = self
            .input
            .iter()
            .map(|x| Signature::from_bytes(&decode_hex_with_prefix(&x)).unwrap())
            .collect();
        let aggregate = match aggregate(&input_signatures) {
            Ok(agg) => agg,
            // handling for zero sized input and output
            Err(_) => {
                assert_eq!(self.output, "~");
                return;
            }
        };
        let expected_aggregate_raw = decode_hex_with_prefix(&self.output);
        let expected_aggregate = Signature::from_bytes(&expected_aggregate_raw).unwrap();
        assert_eq!(aggregate, expected_aggregate)
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct AggVerifyInput {
    pubkeys: Vec<String>,
    messages: Vec<String>,
    signature: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct AggVerifyTestIO {
    input: AggVerifyInput,
    output: bool,
}
impl AggVerifyTestIO {
    fn verify(&self) {
        let pubkeys_result: Result<Vec<PublicKey>, _> = self
            .input
            .pubkeys
            .iter()
            .map(|x| PublicKey::from_bytes(&decode_hex_with_prefix(&x)))
            .collect();
        let pubkeys = match pubkeys_result {
            Ok(pk) => pk,
            // error handling for infinity pub key
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let messages_vec: Vec<_> = self
            .input
            .messages
            .iter()
            .map(|x| decode_hex_with_prefix(x))
            .collect();
        let messages_slice: Vec<&[u8]> = messages_vec.iter().map(|x| x.as_slice()).collect();
        let signature: Signature =
            match Signature::from_bytes(&decode_hex_with_prefix(&self.input.signature)) {
                Ok(sign) => sign,
                // handling for the zero signature case which raises a blst bad encoding error
                Err(_) => {
                    assert!(!self.output);
                    return;
                }
            };
        let verify_result = aggregate_verify(&pubkeys, &messages_slice, signature);
        assert_eq!(verify_result, self.output)
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct FastAggVerifyInput {
    pubkeys: Vec<String>,
    message: String,
    signature: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct FastAggVerifyTestIO {
    input: FastAggVerifyInput,
    output: bool,
}
impl FastAggVerifyTestIO {
    fn verify(&self) {
        let pubkeys_result: Result<Vec<PublicKey>, _> = self
            .input
            .pubkeys
            .iter()
            .map(|x| PublicKey::from_bytes(&decode_hex_with_prefix(&x)))
            .collect();
        let pubkeys: Vec<PublicKey> = match pubkeys_result {
            Ok(pk) => pk,
            // error handling for infinity pub key
            Err(_) => {
                assert!(!self.output);
                return;
            }
        };
        let pubkeys_ref: Vec<&PublicKey> = pubkeys.iter().map(|x| x).collect();
        let message_slice: &[u8] = &decode_hex_with_prefix(&self.input.message);
        let signature: Signature =
            match Signature::from_bytes(&decode_hex_with_prefix(&self.input.signature)) {
                Ok(sk) => sk,
                // error handling for zero signature
                Err(_) => {
                    assert!(!self.output);
                    return;
                }
            };
        let verify_result =
            fast_aggregate_verify(pubkeys_ref.as_slice(), message_slice, &signature);
        assert_eq!(verify_result, self.output)
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct VerifyInput {
    pubkey: String,
    message: String,
    signature: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct VerifyTestIO {
    input: VerifyInput,
    output: bool,
}
impl VerifyTestIO {
    fn verify(&self) {
        let pubkey: PublicKey =
            match PublicKey::from_bytes(&decode_hex_with_prefix(&self.input.pubkey)) {
                Ok(pk) => pk,
                // error handling for infinity pub key
                Err(_) => {
                    assert!(!self.output);
                    return;
                }
            };
        let message_vec: Vec<u8> = decode_hex_with_prefix(&self.input.message);
        let message: &[u8] = &message_vec;
        let signature: Signature =
            match Signature::from_bytes(&decode_hex_with_prefix(&self.input.signature)) {
                Ok(sk) => sk,
                // this is the case where the tampered signature cannot be read in as real signature
                Err(_) => {
                    assert!(!self.output);
                    return;
                }
            };
        let verify_result = signature.verify(&pubkey, message);
        assert_eq!(verify_result, self.output)
    }
}
#[test]
fn test_fast_aggregate_verify() {
    let entries =
        glob("consensus-spec-tests/tests/general/phase0/bls/fast_aggregate_verify/small/**/*.yaml")
            .expect("Failed to read glob pattern");
    for entry in entries {
        let path = entry.unwrap();
        let file = File::open(path).expect("File does not exist");
        let test_case: FastAggVerifyTestIO =
            serde_yaml::from_reader(file).expect("Is not well-formatted yaml");
        test_case.verify()
    }
}
#[test]
fn test_verify() {
    let entries = glob("consensus-spec-tests/tests/general/phase0/bls/verify/small/**/*.yaml")
        .expect("Failed to read glob pattern");
    for entry in entries {
        let path = entry.unwrap();
        let file = File::open(path).expect("File does not exist");
        let test_case: VerifyTestIO =
            serde_yaml::from_reader(file).expect("Is not well-formatted yaml");
        test_case.verify()
    }
}
#[test]
fn test_aggregate_verify() {
    let entries =
        glob("consensus-spec-tests/tests/general/phase0/bls/aggregate_verify/small/**/*.yaml")
            .expect("Failed to read glob pattern");
    for entry in entries {
        let path = entry.unwrap();
        let file = File::open(path).expect("File does not exist");
        let test_case: AggVerifyTestIO =
            serde_yaml::from_reader(file).expect("Is not well-formatted yaml");
        test_case.verify()
    }
}
#[test]
fn test_aggregate() {
    let entries = glob("consensus-spec-tests/tests/general/phase0/bls/aggregate/small/**/*.yaml")
        .expect("Failed to read glob pattern");
    for entry in entries {
        let path = entry.unwrap();
        let file = File::open(path).expect("File does not exist");
        let test_case: AggregatingTestIO =
            serde_yaml::from_reader(file).expect("Is not well-formatted yaml");
        test_case.verify()
    }
}
#[test]
fn test_sign() {
    let entries = glob("consensus-spec-tests/tests/general/phase0/bls/sign/small/**/*.yaml")
        .expect("Failed to read glob pattern");
    for entry in entries {
        let path = entry.unwrap();
        let file = File::open(path).expect("File does not exist");
        let test_case: SigningTestIO =
            serde_yaml::from_reader(file).expect("Is not well-formatted yaml");
        test_case.verify()
    }
}
