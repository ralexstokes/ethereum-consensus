use ethereum_consensus::crypto::{SecretKey, Signature};
use hex;
#[allow(unused_imports)]
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;

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

use std::fmt::Debug;
fn decode_hex_with_prefix<T: AsRef<[u8]> + Debug>(data: T) -> Vec<u8> {
    hex::decode(&data.as_ref()[2..]).expect("is well-formed hex")
}

#[test]
fn try_signing() {
    let path = "consensus-spec-tests/tests/general/phase0/bls/sign/small/sign_case_8cd3d4d0d9a5b265/data.yaml";
    let file = File::open(path).expect("file exists");
    let test_case: TestIO = serde_yaml::from_reader(file).expect("is well-formatted yaml");
    let input = test_case.input;

    let secret_key_bytes = decode_hex_with_prefix(&input.privkey);
    let secret_key = SecretKey::from_bytes(&secret_key_bytes);

    let message_bytes = decode_hex_with_prefix(&input.message);
    let signature = secret_key.sign(&message_bytes);

    let signature_bytes = decode_hex_with_prefix(test_case.output);
    let expected_signature = Signature::from_bytes(&signature_bytes);
    assert_eq!(expected_signature, signature);
}
// #[test]
// fn try_one_case() {
//     // for entry in glob("../consensus-spec-tests/tests/general/altair/bls/eth_aggregate_pubkeys/**/*.yaml").expect("Failed to read glob pattern") {
//     //     match entry {
//     //         Ok(path) => println!("{:?}", path.display()),
//     //         Err(e) => println!("{:?}", e),
//     //     }
//     // }
//     //println! {"ttest"};
//     let paths = "consensus-spec-tests/tests/general/altair/bls/eth_aggregate_pubkeys/small/eth_aggregate_pubkeys_valid_pubkeys/data.yaml";
//     let contents = fs::read_to_string(paths).expect("Something went wrong reading the file");

//     println!("With text:\n{}", contents);

//     let f = std::fs::File::open(paths).expect("Could not open file.");
//     let scrape_config: TestIO = serde_yaml::from_reader(f).expect("Could not read values.");
//     let sigarray: Vec<PublicKey> = scrape_config
//         .input
//         .iter()
//         .map(|x| {
//             PublicKey::from_bytes(hex::decode(x.trim_start_matches("0x")).unwrap().as_slice())
//                 .unwrap()
//         })
//         .collect();

//     // println!("sigarray {:?}", sigarray);
//     let agg: AggregatePublicKey =
//         AggregatePublicKey::aggregate(&[&sigarray[0], &sigarray[1], &sigarray[2]], true).unwrap();
//     //println!("calculated aggregate {:?}", agg);
//     let outkey = PublicKey::from_aggregate(&agg);
//     let outkeystr = PublicKey::fmt::LowerHex(&outkey);
//     println!("this agg \n {:?}", &outkeystr[..96]);

//     println!(
//         "should match \n {:?}",
//         scrape_config.output.trim_start_matches("0x")
//     );
//     assert_eq!(
//         &outkeystr[..96],
//         scrape_config.output.trim_start_matches("0x")
//     )
// }
