use crate::{
    test_case::TestCase,
    test_utils::{load_yaml, Error},
};
use ethereum_consensus::deneb::{
    mainnet as spec,
    polynomial_commitments::{self, blob_to_kzg_commitment, KzgCommitment},
    presets::TRUSTED_SETUP_JSON,
};
use serde::Deserialize;

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    let path = &test.data_path;
    let kzg_settings = polynomial_commitments::kzg_settings_from_json(TRUSTED_SETUP_JSON)?;

    match meta.handler.0.as_str() {
        "blob_to_kzg_commitment" => {
            // Load test case ----
            let path = path.to_string() + "/data.yaml";
            let test_data: serde_yaml::Value = load_yaml(&path);
            let input = test_data.get("input").unwrap();
            let output_yaml = test_data.get("output").unwrap();
            let blob_yaml = input.get("blob").unwrap();
            let blob: spec::Blob = serde_yaml::from_value(blob_yaml.clone()).unwrap();
            let output: polynomial_commitments::KzgCommitment =
                serde_yaml::from_value(output_yaml.clone()).unwrap();

            // TODO: Verify all test case format conditions

            // Run test ----
            let kzg_commitment = blob_to_kzg_commitment(&blob, &kzg_settings).unwrap();
            assert!(kzg_commitment == output);
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
        _ => todo!(),
    }
}

#[derive(Debug, Deserialize)]
pub struct BlobToKzgCommitmentTestCase {
    input: spec::Blob,
    output: Option<KzgCommitment>,
}

impl BlobToKzgCommitmentTestCase {
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/data.yaml";
        load_yaml(&path)
    }

    pub fn run(
        &self,
        blob: &spec::Blob,
        kzg_settings: &polynomial_commitments::KzgSettings,
    ) -> bool {
        let kzg_commitment = blob_to_kzg_commitment(blob, kzg_settings).unwrap();
        println!("Kzg commitment: {}", kzg_commitment);
        &kzg_commitment == self.output.as_ref().unwrap()
    }
}
