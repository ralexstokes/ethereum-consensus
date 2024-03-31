use crate::{
    runners::{gen_exec, gen_match_for},
    test_case::TestCase,
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use ethereum_consensus::Error as SpecError;
use serde::Deserialize;
use ssz_rs::{
    prelude::*,
    proofs::{get_subtree_index, is_valid_merkle_branch_for_generalized_index, log_2, prove},
};

#[derive(Debug, Deserialize)]
pub struct Proof {
    leaf: Node,
    leaf_index: GeneralizedIndex,
    branch: Vec<Node>,
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
                            let root = object.hash_tree_root().unwrap();
                            // test proof matches
                            // NOTE: `0` index hard-coded in test generator
                            let path = &["blob_kzg_commitments".into(), 0.into()];
                            let (computed_proof, witness) = prove(&mut object, path).expect("can prove");
                            assert_eq!(root, witness);
                            assert_eq!(proof.leaf, computed_proof.leaf);
                            assert_eq!(proof.leaf_index, computed_proof.index);
                            assert_eq!(proof.branch, computed_proof.branch);
                            assert!(computed_proof.verify(witness).is_ok());
                            // test generalized index verifier
                            assert!(is_valid_merkle_branch_for_generalized_index(proof.leaf, &proof.branch, proof.leaf_index, root).is_ok());
                            // test regular index verifier
                            is_valid_merkle_branch(
                                proof.leaf,
                                &proof.branch,
                                log_2(proof.leaf_index).unwrap() as usize,
                                get_subtree_index(proof.leaf_index).unwrap(),
                                root,
                            ).map_err(|err| SpecError::from(err).into())
                        }
                    }
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
