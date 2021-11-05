use ethereum_consensus::crypto::{SecretKey};
#[allow(unused_imports)]
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_yaml;


#[derive(Debug, Serialize, Deserialize)]
struct Input {
    privkey: String,
    message: String

}
#[derive(Debug, Serialize, Deserialize)]
struct TestIO {
    input: Input,
    output: String,
}

#[test]

fn try_signing() {

    let paths = "consensus-spec-tests/tests/general/phase0/bls/sign/small/sign_case_8cd3d4d0d9a5b265/data.yaml";
    let f = std::fs::File::open(paths).expect("Could not open file.");
    let scrape_config: TestIO = serde_yaml::from_reader(f).expect("Could not read values.");
    let secretkey = SecretKey::from_bytes(&scrape_config.input.privkey.as_bytes());
    let signed = secretkey.sign(&scrape_config.input.message.as_bytes());
    println!("signed message {:?}",signed);
    println!("should match {:?}",scrape_config.output);
    assert_eq!(false)

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


