// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::rewards::BasicTestCase;

#[test]
fn test_all_balances_too_low_for_reward() {
    let  test_case = BasicTestCase::from("consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/all_balances_too_low_for_reward");

    test_case.execute();
}

#[test]
fn test_empty() {
    let test_case = BasicTestCase::from(
        "consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/empty",
    );

    test_case.execute();
}

#[test]
fn test_full_all_correct() {
    let test_case = BasicTestCase::from(
        "consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/full_all_correct",
    );

    test_case.execute();
}

#[test]
fn test_full_but_partial_participation() {
    let  test_case = BasicTestCase::from("consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/full_but_partial_participation");

    test_case.execute();
}

#[test]
fn test_half_full() {
    let test_case = BasicTestCase::from(
        "consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/half_full",
    );

    test_case.execute();
}

#[test]
fn test_quarter_full() {
    let test_case = BasicTestCase::from(
        "consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/quarter_full",
    );

    test_case.execute();
}

#[test]
fn test_some_very_low_effective_balances_that_attested() {
    let  test_case = BasicTestCase::from("consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/some_very_low_effective_balances_that_attested");

    test_case.execute();
}

#[test]
fn test_some_very_low_effective_balances_that_did_not_attest() {
    let  test_case = BasicTestCase::from("consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/some_very_low_effective_balances_that_did_not_attest");

    test_case.execute();
}

#[test]
fn test_with_exited_validators() {
    let  test_case = BasicTestCase::from("consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/with_exited_validators");

    test_case.execute();
}

#[test]
fn test_with_not_yet_activated_validators() {
    let  test_case = BasicTestCase::from("consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/with_not_yet_activated_validators");

    test_case.execute();
}

#[test]
fn test_with_slashed_validators() {
    let  test_case = BasicTestCase::from("consensus-spec-tests/tests/minimal/altair/rewards/basic/pyspec_tests/with_slashed_validators");

    test_case.execute();
}
