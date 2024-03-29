use crate::{
    test_case::TestCase,
    test_utils::{load_yaml, Error},
};
use ethereum_consensus::deneb::{
    mainnet as spec,
    polynomial_commitments::{blob_to_kzg_commitment, KzgCommitment, KzgSettings},
};
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    let path = &test.data_path;
    match meta.handler.0.as_str() {
        "blob_to_kzg_commitment" => {
            let test_case = BlobToKzgCommitmentTestCase::from(path);
            test_case.run();
            Ok(())
        }
        "compute_kzg_proof" => {
            todo!()
        }
        "verify_kzg_proof" => {
            todo!()
        }
        "compute_blob_kzg_proof" => {
            todo!()
        }
        "verify_blob_kzg_proof" => {
            todo!()
        }
        "verify_blob_kzg_proof_batch" => {
            todo!()
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct BlobToKzgCommitmentInput {
    #[serde_as(deserialize_as = "DefaultOnError")]
    blob: spec::Blob,
    #[serde_as(deserialize_as = "DefaultOnError")]
    kzg_settings: KzgSettings,
}

#[derive(Debug, Deserialize)]
pub struct BlobToKzgCommitmentTestCase {
    input: BlobToKzgCommitmentInput,
    output: KzgCommitment,
}

impl BlobToKzgCommitmentTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }

    pub fn run(&self) -> bool {
        blob_to_kzg_commitment(&self.input.blob, &self.input.kzg_settings).is_ok()
    }
}
