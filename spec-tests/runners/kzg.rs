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
            let test_case = BlobToKzgCommitmentTestCase::from(path);
            test_case.run(&test_case.blob, &kzg_settings);
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
    blob: spec::Blob,
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
