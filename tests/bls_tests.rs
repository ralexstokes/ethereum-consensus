use blst::min_pk::AggregatePublicKey;
use ethereum_consensus::crypto::{SecretKey, Signature, aggregate, aggregate_verify};
use glob::glob;
use hex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;

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
    fn verify_signature(&self) -> bool {
        let input = &self.input;

        let secret_key_bytes = decode_hex_with_prefix(&input.privkey);
        let secret_key = SecretKey::from_bytes(&secret_key_bytes);

        let message_bytes = decode_hex_with_prefix(&input.message);
        let signature = secret_key.sign(&message_bytes);

        let signature_bytes = decode_hex_with_prefix(&self.output);
        let expected_signature = Signature::from_bytes(&signature_bytes);

        signature == expected_signature
    }
}
use std::fmt::Debug;
fn decode_hex_with_prefix<T: AsRef<[u8]> + Debug>(data: T) -> Vec<u8> {
    hex::decode(&data.as_ref()[2..]).expect("is well-formed hex")
}

#[derive(Debug, Serialize, Deserialize)]

struct AggregatingTestIO {
    input: Vec<String>,
    output: String,
}

impl AggregatingTestIO {
    fn verify_aggregate(&self) -> bool {
        let input_signatures: Vec<Signature> = self
        .input
        .iter()
        .map(|x| Signature::from_bytes(&decode_hex_with_prefix(&x)))
        .collect();
    let aggregate = aggregate(&input_signatures).expect("aggregation failed");
    let expected_aggregate_raw = decode_hex_with_prefix(&self.output);
    let expected_aggregate = Signature::from_bytes(&expected_aggregate_raw);
    aggregate == expected_aggregate
    }
}
#[test]

fn try_aggregating() {
    // let path = "consensus-spec-tests/tests/general/phase0/bls/aggregate/small/aggregate_0x0000000000000000000000000000000000000000000000000000000000000000/data.yaml";
    for entry in glob("consensus-spec-tests/tests/general/phase0/bls/aggregate/small/**/*.yaml")
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                let file = File::open(path).expect("file exists");
                let test_case: AggregatingTestIO =
                serde_yaml::from_reader(file).expect("is well-formatted yaml");
                assert!(test_case.verify_aggregate())
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

#[test]
fn try_signing() {
    for entry in glob("consensus-spec-tests/tests/general/phase0/bls/sign/small/**/*.yaml")
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                let file = File::open(path).expect("file exists");
                let test_case: SigningTestIO =
                    serde_yaml::from_reader(file).expect("is well-formatted yaml");
                assert!(test_case.verify_signature())
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
