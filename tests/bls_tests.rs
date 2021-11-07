use ethereum_consensus::crypto::{SecretKey, Signature};
use hex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use glob::glob;

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    privkey: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestIO {
    input: Input,
    output: String,
}

impl TestIO {
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

#[test]
fn try_signing() {
    for entry in glob("consensus-spec-tests/tests/general//phase0/bls/sign/small/**/*.yaml").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                let file = File::open(path).expect("file exists");
                let test_case: TestIO = serde_yaml::from_reader(file).expect("is well-formatted yaml");
                assert!(test_case.verify_signature())
            },
            Err(e) => println!("{:?}", e),
        }
    }
}