use blst::min_pk::{AggregatePublicKey, PublicKey, SecretKey};
use ethereum_consensus::crypto::{aggregate, aggregate_verify, Signature};
#[allow(unused_imports)]
use rand::Rng;
// use glob::glob;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs;
// use hex::decode;

#[derive(Debug, Serialize, Deserialize)]
struct TestIO {
    input: Vec<String>,
    output: String,
}

#[test]
fn try_one_case() {
    // for entry in glob("../consensus-spec-tests/tests/general/altair/bls/eth_aggregate_pubkeys/**/*.yaml").expect("Failed to read glob pattern") {
    //     match entry {
    //         Ok(path) => println!("{:?}", path.display()),
    //         Err(e) => println!("{:?}", e),
    //     }
    // }
    //println! {"ttest"};
    let paths = "consensus-spec-tests/tests/general/altair/bls/eth_aggregate_pubkeys/small/eth_aggregate_pubkeys_valid_pubkeys/data.yaml";
    let contents = fs::read_to_string(paths).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    let f = std::fs::File::open(paths).expect("Could not open file.");
    let scrape_config: TestIO = serde_yaml::from_reader(f).expect("Could not read values.");
    let sigarray: Vec<PublicKey> = scrape_config
        .input
        .iter()
        .map(|x| {
            PublicKey::from_bytes(hex::decode(x.trim_start_matches("0x")).unwrap().as_slice())
                .unwrap()
        })
        .collect();

    // println!("sigarray {:?}", sigarray);
    let agg: AggregatePublicKey =
        AggregatePublicKey::aggregate(&[&sigarray[0], &sigarray[1], &sigarray[2]], true).unwrap();
    //println!("calculated aggregate {:?}", agg);
    let outkey = PublicKey::from_aggregate(&agg);
    let outkeystr = hex::encode(PublicKey::serialize(&outkey));
    println!("this agg \n {:?}", &outkeystr[..96]);

    // let answer: AggregatePublicKey = AggregatePublicKey::from_public_key(
    //     &PublicKey::from_bytes(
    //         hex::decode(scrape_config.output.trim_start_matches("0x"))
    //             .unwrap()
    //             .as_slice(),
    //     )
    //     .unwrap(),
    // );
    println!(
        "should match \n {:?}",
        scrape_config.output.trim_start_matches("0x")
    );
    assert_eq!(
        &outkeystr[..96],
        scrape_config.output.trim_start_matches("0x")
    )
}
