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
    // a small check for empty strings
    if data.as_ref().len() >=2 {
    hex::decode(&data.as_ref()[2..]).expect("is well-formed hex")
    } else {
        hex::decode(&data.as_ref()).expect("is well-formed hex")}

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
    fn verify_signature(&self) -> bool {
        let input = &self.input;

        let secret_key_bytes = decode_hex_with_prefix(&input.privkey);
        let secret_key = match SecretKey::from_bytes(&secret_key_bytes) {
            Ok(sk) => sk,
            Err(_) => {
                // this is the case empty secret key case
                if self.output == String::from("~") {
                    return true;
                } else {
                    return false;
                }
            }
        };

        let message_bytes = decode_hex_with_prefix(&input.message);
        let signature = secret_key.sign(&message_bytes);

        let signature_bytes = decode_hex_with_prefix(&self.output);
        let expected_signature = Signature::from_bytes(&signature_bytes).unwrap();

        signature == expected_signature
    }
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
            .map(|x| Signature::from_bytes(&decode_hex_with_prefix(&x)).unwrap())
            .collect();
        let aggregate = match aggregate(&input_signatures){
            Ok(agg) => agg,
            // handling for zero sized input and output
            Err(_) => {
                if self.output == String::from("~") {
                    return true
                } else {
                    return false
                }
            },
        };
        let expected_aggregate_raw = decode_hex_with_prefix(&self.output);
        let expected_aggregate = Signature::from_bytes(&expected_aggregate_raw).unwrap();
        aggregate == expected_aggregate
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
    fn aggregate_verify(&self) -> bool {
        let pubkeys: Vec<PublicKey> = self
            .input
            .pubkeys
            .iter()
            .map(|x| PublicKey::from_bytes(&decode_hex_with_prefix(&x)).unwrap())
            .collect();
        let messages_vec: Vec<_> = self
            .input
            .messages
            .iter()
            .map(|x| decode_hex_with_prefix(x))
            .collect();
        let messages_slice: Vec<&[u8]> = messages_vec.iter().map(|x| x.as_slice()).collect();
        let signature: Signature =
            match Signature::from_bytes(&decode_hex_with_prefix(&self.input.signature)){
                Ok(sign) => sign,
                Err(_) => {
                    // handling for the zero signature case which raises a blst bad encoding error
                    if self.output == false {
                        return true
                    } else {
                        return false
                    }
                },
            };
        let verify_result = aggregate_verify(&pubkeys, messages_slice.as_ref(), signature);
        verify_result == self.output
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
    fn fast_aggregate_verify(&self) -> bool {
        let pubkeys: Vec<PublicKey> = self
            .input
            .pubkeys
            .iter()
            .map(|x| PublicKey::from_bytes(&decode_hex_with_prefix(&x)).unwrap())
            .collect();
        // println!("{:?}",pubkeys);
        let message_slice: &[u8] = &decode_hex_with_prefix(&self.input.message);
        let signature: Signature =
            Signature::from_bytes(&decode_hex_with_prefix(&self.input.signature)).unwrap();
        let verify_result = fast_aggregate_verify(&pubkeys, message_slice, signature);
        // println!("{}",verify_result);
        verify_result == self.output
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
    fn verify(&self) -> bool {
        let pubkey: PublicKey =
            PublicKey::from_bytes(&decode_hex_with_prefix(&self.input.pubkey)).unwrap();
        let message_vec: Vec<u8> = decode_hex_with_prefix(&self.input.message);
        let message: &[u8] = message_vec.as_ref();
        let signature: Signature =
            match Signature::from_bytes(&decode_hex_with_prefix(&self.input.signature)) {
                Ok(sk) => sk,
                Err(_) => {
                    // this is the case where the tampered signature cannot be read in as real signature
                    if self.output == false {
                        return true;
                    } else {
                        return false;
                    }
                }
            };
        let verify_result = signature.verify(&pubkey, message);
        verify_result == self.output
    }
}

#[test]
#[cfg(feature = "ef_spec_tests")]
fn test_fast_aggregate_verify() {
    for entry in
        glob("consensus-spec-tests/tests/general/phase0/bls/fast_aggregate_verify/small/**/*.yaml")
            .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                let file = File::open(path).expect("file exists");
                let test_case: FastAggVerifyTestIO =
                    serde_yaml::from_reader(file).expect("is well-formatted yaml");
                assert!(test_case.fast_aggregate_verify())
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

#[test]
#[ignore]
#[cfg(feature = "ef_spec_tests")]
fn test_verify() {
    for entry in glob("consensus-spec-tests/tests/general/phase0/bls/verify/small/**/*.yaml")
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                let file = File::open(path).expect("file exists");
                let test_case: VerifyTestIO =
                    serde_yaml::from_reader(file).expect("is well-formatted yaml");
                assert!(test_case.verify())
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
#[test]
#[ignore]
#[cfg(feature = "ef_spec_tests")]
fn test_aggregate_verify() {
    for entry in
        glob("consensus-spec-tests/tests/general/phase0/bls/aggregate_verify/small/**/*.yaml")
            .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                let file = File::open(path).expect("file exists");
                let test_case: AggVerifyTestIO =
                    serde_yaml::from_reader(file).expect("is well-formatted yaml");
                assert!(test_case.aggregate_verify())
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

#[test]
#[ignore]
#[cfg(feature = "ef_spec_tests")]

fn test_aggregate() {
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
#[ignore]
#[cfg(feature = "ef_spec_tests")]

fn test_sign() {
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
