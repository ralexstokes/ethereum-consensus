use crate::{
    test_case::TestCase,
    test_utils::{load_yaml, Error},
};
use ethereum_consensus::deneb::{
    mainnet::Blob,
    polynomial_commitments::{
        blob_to_kzg_commitment, kzg_settings_from_json, CKzgError,
        Error as PolynomialCommitmentsError, KzgCommitment,
    },
    presets::TRUSTED_SETUP_JSON,
};

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    let path = &test.data_path;
    let kzg_settings = kzg_settings_from_json(TRUSTED_SETUP_JSON)?;

    match meta.handler.0.as_str() {
        "blob_to_kzg_commitment" => {
            // Load test case ----
            let path = path.to_string() + "/data.yaml";
            let test_data: serde_yaml::Value = load_yaml(&path);
            let input = test_data.get("input").unwrap();
            let output_yaml = test_data.get("output").unwrap();
            let blob_yaml = input.get("blob").unwrap();

            let input_result: Result<Blob, _> = serde_yaml::from_value(blob_yaml.clone());
            let output_result: Result<Option<KzgCommitment>, _> =
                serde_yaml::from_value(output_yaml.clone());

            match (input_result, output_result) {
                (Ok(blob), Ok(Some(expected))) => {
                    let kzg_commitment = blob_to_kzg_commitment(&blob, &kzg_settings).unwrap();
                    assert!(kzg_commitment == expected);
                    Ok(())
                }
                (Err(_), Ok(None)) => {
                    // Expected state for failed test case
                    Ok(())
                }
                (Ok(blob), Ok(None)) => {
                    let result = blob_to_kzg_commitment(&blob, &kzg_settings);
                    assert!(matches!(
                        result,
                        Err(PolynomialCommitmentsError::CKzg(CKzgError::CError(..)))
                    ));
                    Ok(())
                }
                _ => unreachable!("not possible"),
            }
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
/*
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
 */
