// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::rewards::LeakHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_full_random_ten_epoch_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_random_ten_epoch_leak");
    test_case.execute();
}

#[test]
fn test_full_random_leak() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_random_leak",
    );
    test_case.execute();
}

#[test]
fn test_quarter_full_leak() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/quarter_full_leak",
    );
    test_case.execute();
}

#[test]
fn test_with_not_yet_activated_validators_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/with_not_yet_activated_validators_leak");
    test_case.execute();
}

#[test]
fn test_full_half_correct_target_incorrect_head_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_half_correct_target_incorrect_head_leak");
    test_case.execute();
}

#[test]
fn test_one_attestation_one_correct_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/one_attestation_one_correct_leak");
    test_case.execute();
}

#[test]
fn test_half_full_leak() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/half_full_leak",
    );
    test_case.execute();
}

#[test]
fn test_full_correct_target_incorrect_head_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_correct_target_incorrect_head_leak");
    test_case.execute();
}

#[test]
fn test_some_very_low_effective_balances_that_did_not_attest_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/some_very_low_effective_balances_that_did_not_attest_leak");
    test_case.execute();
}

#[test]
fn test_some_very_low_effective_balances_that_attested_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/some_very_low_effective_balances_that_attested_leak");
    test_case.execute();
}

#[test]
fn test_full_random_seven_epoch_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_random_seven_epoch_leak");
    test_case.execute();
}

#[test]
fn test_full_leak() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_leak",
    );
    test_case.execute();
}

#[test]
fn test_full_but_partial_participation_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_but_partial_participation_leak");
    test_case.execute();
}

#[test]
fn test_with_slashed_validators_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/with_slashed_validators_leak");
    test_case.execute();
}

#[test]
fn test_with_exited_validators_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/with_exited_validators_leak");
    test_case.execute();
}

#[test]
fn test_full_half_incorrect_target_incorrect_head_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_half_incorrect_target_incorrect_head_leak");
    test_case.execute();
}

#[test]
fn test_full_half_incorrect_target_correct_head_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/full_half_incorrect_target_correct_head_leak");
    test_case.execute();
}

#[test]
fn test_empty_leak() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/leak/pyspec_tests/empty_leak",
    );
    test_case.execute();
}
