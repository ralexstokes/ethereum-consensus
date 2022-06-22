// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::VoluntaryExitHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_validator_already_exited() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/validator_already_exited");
    test_case.execute();
}

#[test]
fn test_default_exit_epoch_subsequent_exit() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/default_exit_epoch_subsequent_exit");
    test_case.execute();
}

#[test]
fn test_validator_not_active_long_enough() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/validator_not_active_long_enough");
    test_case.execute();
}

#[test]
fn test_validator_not_active() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/validator_not_active");
    test_case.execute();
}

#[test]
fn test_validator_invalid_validator_index() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/validator_invalid_validator_index");
    test_case.execute();
}

#[test]
fn test_invalid_signature() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/invalid_signature");
    test_case.execute();
}

#[test]
fn test_validator_exit_in_future() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/validator_exit_in_future");
    test_case.execute();
}

#[test]
fn test_success_exit_queue_min_churn() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/success_exit_queue__min_churn");
    test_case.execute();
}

#[test]
fn test_success() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/success");
    test_case.execute();
}

#[test]
fn test_success_exit_queue_scaled_churn() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/voluntary_exit/pyspec_tests/success_exit_queue__scaled_churn");
    test_case.execute();
}
