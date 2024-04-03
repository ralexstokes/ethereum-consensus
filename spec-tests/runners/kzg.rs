use crate::{
    test_case::TestCase,
    test_utils::{load_yaml, Error},
};
use ethereum_consensus::deneb::{
    mainnet::Blob,
    polynomial_commitments::{
        blob_to_kzg_commitment, compute_kzg_proof, kzg_settings_from_json, CKzgError,
        Error as PolynomialCommitmentsError, FieldElement, KzgCommitment, KzgProof,
        ProofAndEvaluation,
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
            let input_yaml = test_data.get("input").unwrap();
            let output_yaml = test_data.get("output").unwrap();
            let blob_yaml = input_yaml.get("blob").unwrap();

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
            // Load test case ----
            let path = path.to_string() + "/data.yaml";
            let test_data: serde_yaml::Value = load_yaml(&path);
            let input_yaml = test_data.get("input").unwrap();
            let output_yaml = test_data.get("output").unwrap();
            let blob_yaml = input_yaml.get("blob").unwrap();
            let z_yaml = input_yaml.get("z").unwrap();

            let input_blob_result: Result<Blob, _> = serde_yaml::from_value(blob_yaml.clone());
            let input_z_result: Result<FieldElement, _> = serde_yaml::from_value(z_yaml.clone());
            let output_result: Result<Option<(KzgProof, FieldElement)>, _> =
                serde_yaml::from_value(output_yaml.clone());

            match (input_blob_result, input_z_result, output_result) {
                // All maps for yaml file deserialized correctly
                (Ok(blob), Ok(z), Ok(Some(expected))) => {
                    let proof_and_evaluation = compute_kzg_proof(&blob, &z, &kzg_settings).unwrap();
                    let expected = ProofAndEvaluation { proof: expected.0, evaluation: expected.1 };
                    assert_eq!(proof_and_evaluation, expected);
                    Ok(())
                }
                (Err(_), Ok(_), Ok(None)) => {
                    // Expected state for failed test case - invalid length blob
                    Ok(())
                }
                (Ok(blob), Ok(z), Ok(None)) => {
                    let result = compute_kzg_proof(&blob, &z, &kzg_settings);
                    assert!(matches!(
                        result,
                        Err(PolynomialCommitmentsError::CKzg(CKzgError::CError(..)))
                    ));
                    Ok(())
                }

                (Ok(_), Err(_), Ok(None)) => {
                    // Expected state for failed test case - invalid evaluation point
                    Ok(())
                }
                _ => unreachable!("not possible"),
            }
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
