// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::JustificationAndFinalizationHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_234_poor_support() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/234_poor_support");
    test_case.execute();
}

#[test]
fn test_12_ok_support() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/12_ok_support");
    test_case.execute();
}

#[test]
fn test_12_ok_support_messed_target() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/12_ok_support_messed_target");
    test_case.execute();
}

#[test]
fn test_123_ok_support() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/123_ok_support");
    test_case.execute();
}

#[test]
fn test_234_ok_support() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/234_ok_support");
    test_case.execute();
}

#[test]
fn test_23_ok_support() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/23_ok_support");
    test_case.execute();
}

#[test]
fn test_23_poor_support() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/23_poor_support");
    test_case.execute();
}

#[test]
fn test_balance_threshold_with_exited_validators() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/balance_threshold_with_exited_validators");
    test_case.execute();
}

#[test]
fn test_123_poor_support() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/123_poor_support");
    test_case.execute();
}

#[test]
fn test_12_poor_support() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/justification_and_finalization/pyspec_tests/12_poor_support");
    test_case.execute();
}
