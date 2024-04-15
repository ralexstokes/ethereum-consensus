use crate::{
    test_case::TestCase,
    test_utils::{load_yaml, Error},
};
use ethereum_consensus::deneb::{
    mainnet::Blob,
    polynomial_commitments::{
        blob_to_kzg_commitment, compute_blob_kzg_proof, compute_kzg_proof, verify_blob_kzg_proof,
        verify_blob_kzg_proof_batch, verify_kzg_proof, FieldElement, KzgCommitment, KzgProof,
        KzgSettings, ProofAndEvaluation,
    },
};

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let kzg_settings = &test.context().kzg_settings;

    match test.meta.handler.0.as_str() {
        "blob_to_kzg_commitment" => run_blob_to_kzg_commitment_test(test, &kzg_settings),
        "compute_kzg_proof" => run_compute_kzg_proof_test(test, &kzg_settings),
        "verify_kzg_proof" => run_verify_kzg_proof_test(test, &kzg_settings),
        "compute_blob_kzg_proof" => run_compute_blob_kzg_proof_test(test, &kzg_settings),
        "verify_blob_kzg_proof" => run_verify_blob_kzg_proof_test(test, &kzg_settings),
        "verify_blob_kzg_proof_batch" => run_verify_blob_kzg_proof_batch_test(test, &kzg_settings),
        handler => unreachable!("no tests for {handler}"),
    }
}

fn run_blob_to_kzg_commitment_test(
    test: &TestCase,
    kzg_settings: &KzgSettings,
) -> Result<(), Error> {
    let path = &test.data_path;
    // Load test case ----
    let path = path.to_string() + "/data.yaml";
    let test_data: serde_yaml::Value = load_yaml(&path);
    let input_yaml = test_data.get("input").unwrap();
    let blob_yaml = input_yaml.get("blob").unwrap();
    let output_yaml = test_data.get("output").unwrap();
    let output: Option<KzgCommitment> = serde_yaml::from_value(output_yaml.clone()).unwrap();

    // Check the deserialization of input(s)
    let blob: Blob = match serde_yaml::from_value(blob_yaml.clone()) {
        Ok(blob) => blob,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let result = blob_to_kzg_commitment(&blob, kzg_settings);
    if let Some(expected_commitment) = output {
        // some `output` was present, use inner value to determine if the spec code should succeed
        // or fail
        match result {
            Ok(commitment) => {
                assert_eq!(commitment, expected_commitment);
                Ok(())
            }
            Err(_) => Ok(()),
        }
    } else {
        // `output` is `null`, implying the spec code should always fail
        let result = blob_to_kzg_commitment(&blob, kzg_settings);
        assert!(result.is_err());
        Ok(())
    }
}

fn run_compute_kzg_proof_test(test: &TestCase, kzg_settings: &KzgSettings) -> Result<(), Error> {
    let path = &test.data_path;
    let path = path.to_string() + "/data.yaml";
    let test_data: serde_yaml::Value = load_yaml(&path);
    let input_yaml = test_data.get("input").unwrap();
    let blob_yaml = input_yaml.get("blob").unwrap();
    let z_yaml = input_yaml.get("z").unwrap();
    let output_yaml = test_data.get("output").unwrap();
    let output: Option<(KzgProof, FieldElement)> =
        serde_yaml::from_value(output_yaml.clone()).unwrap();

    let blob: Blob = match serde_yaml::from_value(blob_yaml.clone()) {
        Ok(blob) => blob,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let z = match serde_yaml::from_value(z_yaml.clone()) {
        Ok(z) => z,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let result = compute_kzg_proof(&blob, &z, kzg_settings);
    if let Some(expected_proof_and_evaluation) = output {
        match result {
            Ok(proof_and_evaluation) => {
                let expected_proof_and_evaluation = ProofAndEvaluation {
                    proof: expected_proof_and_evaluation.0,
                    evaluation: expected_proof_and_evaluation.1,
                };
                assert_eq!(proof_and_evaluation, expected_proof_and_evaluation);
                Ok(())
            }
            Err(_) => Ok(()),
        }
    } else {
        let result = compute_kzg_proof(&blob, &z, kzg_settings);
        assert!(result.is_err());
        Ok(())
    }
}

fn run_verify_kzg_proof_test(test: &TestCase, kzg_settings: &KzgSettings) -> Result<(), Error> {
    let path = &test.data_path;
    let path = path.to_string() + "/data.yaml";
    let test_data: serde_yaml::Value = load_yaml(&path);
    let input_yaml = test_data.get("input").unwrap();
    let commitment_yaml = input_yaml.get("commitment").unwrap();
    let z_yaml = input_yaml.get("z").unwrap();
    let y_yaml = input_yaml.get("y").unwrap();
    let proof_yaml = input_yaml.get("proof").unwrap();
    let output_yaml = test_data.get("output").unwrap();
    let output: Option<bool> = serde_yaml::from_value(output_yaml.clone()).unwrap();

    let commitment = match serde_yaml::from_value(commitment_yaml.clone()) {
        Ok(commitment) => commitment,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let z = match serde_yaml::from_value(z_yaml.clone()) {
        Ok(z) => z,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let y = match serde_yaml::from_value(y_yaml.clone()) {
        Ok(y) => y,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let proof = match serde_yaml::from_value(proof_yaml.clone()) {
        Ok(proof) => proof,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let result = verify_kzg_proof(&commitment, &z, &y, &proof, kzg_settings);
    if let Some(expected_validity) = output {
        if expected_validity {
            assert!(result.is_ok());
            Ok(())
        } else {
            assert!(result.is_err());
            Ok(())
        }
    } else {
        let result = verify_kzg_proof(&commitment, &z, &y, &proof, kzg_settings);
        assert!(result.is_err());
        Ok(())
    }
}

fn run_compute_blob_kzg_proof_test(
    test: &TestCase,
    kzg_settings: &KzgSettings,
) -> Result<(), Error> {
    let path = &test.data_path;
    let path = path.to_string() + "/data.yaml";
    let test_data: serde_yaml::Value = load_yaml(&path);
    let input_yaml = test_data.get("input").unwrap();
    let blob_yaml = input_yaml.get("blob").unwrap();
    let commitment_yaml = input_yaml.get("commitment").unwrap();
    let output_yaml = test_data.get("output").unwrap();
    let output: Option<KzgProof> = serde_yaml::from_value(output_yaml.clone()).unwrap();

    let blob: Blob = match serde_yaml::from_value(blob_yaml.clone()) {
        Ok(blob) => blob,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let commitment = match serde_yaml::from_value(commitment_yaml.clone()) {
        Ok(commitment) => commitment,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let result = compute_blob_kzg_proof(&blob, &commitment, kzg_settings);
    if let Some(expected_proof) = output {
        match result {
            Ok(proof) => {
                assert_eq!(proof, expected_proof);
                Ok(())
            }
            Err(_) => Ok(()),
        }
    } else {
        let result = compute_blob_kzg_proof(&blob, &commitment, kzg_settings);
        assert!(result.is_err());
        Ok(())
    }
}

fn run_verify_blob_kzg_proof_test(
    test: &TestCase,
    kzg_settings: &KzgSettings,
) -> Result<(), Error> {
    let path = &test.data_path;
    let path = path.to_string() + "/data.yaml";
    let test_data: serde_yaml::Value = load_yaml(&path);
    let input_yaml = test_data.get("input").unwrap();
    let blob_yaml = input_yaml.get("blob").unwrap();
    let commitment_yaml = input_yaml.get("commitment").unwrap();
    let proof_yaml = input_yaml.get("proof").unwrap();
    let output_yaml = test_data.get("output").unwrap();
    let output: Option<bool> = serde_yaml::from_value(output_yaml.clone()).unwrap();

    let blob: Blob = match serde_yaml::from_value(blob_yaml.clone()) {
        Ok(blob) => blob,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let commitment = match serde_yaml::from_value(commitment_yaml.clone()) {
        Ok(commitment) => commitment,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let proof = match serde_yaml::from_value(proof_yaml.clone()) {
        Ok(proof) => proof,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let result = verify_blob_kzg_proof(&blob, &commitment, &proof, kzg_settings);
    if let Some(expected_validity) = output {
        if expected_validity {
            assert!(result.is_ok());
            Ok(())
        } else {
            assert!(result.is_err());
            Ok(())
        }
    } else {
        let result = verify_blob_kzg_proof(&blob, &commitment, &proof, kzg_settings);
        assert!(result.is_err());
        Ok(())
    }
}

fn run_verify_blob_kzg_proof_batch_test(
    test: &TestCase,
    kzg_settings: &KzgSettings,
) -> Result<(), Error> {
    let path = &test.data_path;
    let path = path.to_string() + "/data.yaml";
    let test_data: serde_yaml::Value = load_yaml(&path);
    let input_yaml = test_data.get("input").unwrap();
    let blobs_yaml = input_yaml.get("blobs").unwrap();
    let commitments_yaml = input_yaml.get("commitments").unwrap();
    let proofs_yaml = input_yaml.get("proofs").unwrap();
    let output_yaml = test_data.get("output").unwrap();
    let output: Option<bool> = serde_yaml::from_value(output_yaml.clone()).unwrap();

    let blobs: Vec<Blob> = match serde_yaml::from_value(blobs_yaml.clone()) {
        Ok(blobs) => blobs,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };
    let commitments: Vec<KzgCommitment> = match serde_yaml::from_value(commitments_yaml.clone()) {
        Ok(commitments) => commitments,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };
    let proofs: Vec<KzgProof> = match serde_yaml::from_value(proofs_yaml.clone()) {
        Ok(proofs) => proofs,
        Err(_) => {
            assert!(output.is_none());
            return Ok(());
        }
    };

    let result = verify_blob_kzg_proof_batch(&blobs, &commitments, &proofs, kzg_settings);
    if let Some(expected_validity) = output {
        if expected_validity {
            assert!(result.is_ok());
            Ok(())
        } else {
            assert!(result.is_err());
            Ok(())
        }
    } else {
        let result = verify_blob_kzg_proof_batch(&blobs, &commitments, &proofs, kzg_settings);
        assert!(result.is_err());
        Ok(())
    }
}
