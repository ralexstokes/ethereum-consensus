use crate::{
    runners::{gen_exec, gen_match_for},
    test_case::TestCase,
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use ethereum_consensus::{
    primitives::{Bytes32, Root},
    Error as SpecError,
};
use serde::Deserialize;
use ssz_rs::{
    prelude::*,
    proofs::{get_subtree_index, log_2},
};

#[derive(Debug, Deserialize)]
pub struct Proof {
    leaf: Root,
    leaf_index: GeneralizedIndex,
    branch: Vec<Bytes32>,
}

fn load_test<O: ssz_rs::Deserialize>(test_case_path: &str) -> (O, Proof) {
    let path = test_case_path.to_string() + "/object.ssz_snappy";
    let object: O = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/proof.yaml";
    let proof: Proof = load_yaml(&path);

    (object, proof)
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "single_merkle_proof" => {
            gen_match_for! {
                test,
                (mainnet, deneb),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_test,
                        |(mut object, proof): (spec::BeaconBlockBody, Proof), _| {
                            is_valid_merkle_branch(
                                proof.leaf,
                                &proof.branch,
                                log_2(proof.leaf_index).unwrap() as usize,
                                get_subtree_index(proof.leaf_index).unwrap(),
                                object.hash_tree_root().unwrap()
                            ).map_err(|err| SpecError::from(err).into())
                        }
                    }
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
