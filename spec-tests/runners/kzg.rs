use crate::{
    test_case::TestCase,
    test_utils::{load_yaml, Error},
};
use ethereum_consensus::deneb::{
    mainnet::Blob,
    polynomial_commitments::{
        blob_to_kzg_commitment, compute_kzg_proof, kzg_settings_from_json, verify_kzg_proof,
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
            let blob_yaml = input_yaml.get("blob").unwrap();
            let output_yaml = test_data.get("output").unwrap();

            let input_result: Result<Blob, _> = serde_yaml::from_value(blob_yaml.clone());
            let output_result: Result<Option<KzgCommitment>, _> =
                serde_yaml::from_value(output_yaml.clone());

            match (input_result, output_result) {
                (Ok(blob), Ok(Some(expected_commmitment))) => {
                    let kzg_commitment = blob_to_kzg_commitment(&blob, &kzg_settings).unwrap();
                    assert!(kzg_commitment == expected_commmitment);
                    Ok(())
                }
                (Err(_), Ok(None)) => {
                    // Note: Expected state for invalid length blob
                    Ok(())
                }
                (Ok(blob), Ok(None)) => {
                    let result = blob_to_kzg_commitment(&blob, &kzg_settings);
                    assert!(matches!(result, Err(PolynomialCommitmentsError::CKzg(..))));
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
            let blob_yaml = input_yaml.get("blob").unwrap();
            let z_yaml = input_yaml.get("z").unwrap();
            let output_yaml = test_data.get("output").unwrap();

            let input_blob_result: Result<Blob, _> = serde_yaml::from_value(blob_yaml.clone());
            let input_z_result: Result<FieldElement, _> = serde_yaml::from_value(z_yaml.clone());
            let output_result: Result<Option<(KzgProof, FieldElement)>, _> =
                serde_yaml::from_value(output_yaml.clone());

            match (input_blob_result, input_z_result, output_result) {
                // Note: All maps for yaml file deserialized correctly
                (Ok(blob), Ok(z), Ok(Some(expected_proof_and_evaluation))) => {
                    let proof_and_evaluation = compute_kzg_proof(&blob, &z, &kzg_settings).unwrap();
                    let expected_proof_and_evaluation = ProofAndEvaluation {
                        proof: expected_proof_and_evaluation.0,
                        evaluation: expected_proof_and_evaluation.1,
                    };
                    assert_eq!(proof_and_evaluation, expected_proof_and_evaluation);
                    Ok(())
                }
                (Err(_), Ok(_), Ok(None)) => {
                    // Note: Expected state for invalid length blob
                    Ok(())
                }
                (Ok(blob), Ok(z), Ok(None)) => {
                    let result = compute_kzg_proof(&blob, &z, &kzg_settings);
                    assert!(matches!(result, Err(PolynomialCommitmentsError::CKzg(..))));
                    Ok(())
                }
                (Ok(_), Err(_), Ok(None)) => {
                    // Note: Expected state for invalid evaluation point
                    Ok(())
                }
                _ => unreachable!("not possible"),
            }
        }
        "verify_kzg_proof" => {
            // Load test case ----
            let path = path.to_string() + "/data.yaml";
            let test_data: serde_yaml::Value = load_yaml(&path);
            let input_yaml = test_data.get("input").unwrap();
            let commitment_yaml = input_yaml.get("commitment").unwrap();
            let z_yaml = input_yaml.get("z").unwrap();
            let y_yaml = input_yaml.get("y").unwrap();
            let proof_yaml = input_yaml.get("proof").unwrap();
            let output_yaml = test_data.get("output").unwrap();

            let input_commitment_result: Result<KzgCommitment, _> =
                serde_yaml::from_value(commitment_yaml.clone());
            let input_z_result: Result<FieldElement, _> = serde_yaml::from_value(z_yaml.clone());
            let input_y_result: Result<FieldElement, _> = serde_yaml::from_value(y_yaml.clone());
            let input_proof_result: Result<KzgProof, _> =
                serde_yaml::from_value(proof_yaml.clone());
            let output_result: Result<Option<bool>, _> =
                serde_yaml::from_value(output_yaml.clone());

            match (
                input_commitment_result,
                input_z_result,
                input_y_result,
                input_proof_result,
                output_result,
            ) {
                (
                    Ok(commitment),
                    Ok(z),
                    Ok(y),
                    Ok(proof),
                    Ok(Some(expected_verification_status)),
                ) => {
                    assert!(
                        verify_kzg_proof(&commitment, &z, &y, &proof, &kzg_settings).is_ok()
                            == expected_verification_status
                    );
                    Ok(())
                }
                // Note: If the commitment or proof or z or y is invalid...
                (Ok(commitment), Ok(z), Ok(y), Ok(proof), Ok(None)) => {
                    let result = verify_kzg_proof(&commitment, &z, &y, &proof, &kzg_settings);
                    assert!(matches!(result, Err(..)));
                    Ok(())
                }

                // TODO: Implement remaining test case states
                _ => unreachable!("not possible"),
            }
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
        handler => unreachable!("no tests for {handler}"),
    }
}
