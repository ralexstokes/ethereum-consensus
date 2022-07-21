// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::genesis::ValidityTestCase;

#[test]
fn test_is_valid_genesis_state_false_invalid_timestamp() {
    let  test_case = ValidityTestCase::from("consensus-spec-tests/tests/minimal/phase0/genesis/validity/pyspec_tests/is_valid_genesis_state_false_invalid_timestamp");

    test_case.execute();
}

#[test]
fn test_is_valid_genesis_state_false_not_enough_validator() {
    let  test_case = ValidityTestCase::from("consensus-spec-tests/tests/minimal/phase0/genesis/validity/pyspec_tests/is_valid_genesis_state_false_not_enough_validator");

    test_case.execute();
}

#[test]
fn test_is_valid_genesis_state_true() {
    let  test_case = ValidityTestCase::from("consensus-spec-tests/tests/minimal/phase0/genesis/validity/pyspec_tests/is_valid_genesis_state_true");

    test_case.execute();
}

#[test]
fn test_is_valid_genesis_state_true_more_balance() {
    let  test_case = ValidityTestCase::from("consensus-spec-tests/tests/minimal/phase0/genesis/validity/pyspec_tests/is_valid_genesis_state_true_more_balance");

    test_case.execute();
}

#[test]
fn test_is_valid_genesis_state_true_one_more_validator() {
    let  test_case = ValidityTestCase::from("consensus-spec-tests/tests/minimal/phase0/genesis/validity/pyspec_tests/is_valid_genesis_state_true_one_more_validator");

    test_case.execute();
}
