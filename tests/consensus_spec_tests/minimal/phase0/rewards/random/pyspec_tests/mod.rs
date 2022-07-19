// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::rewards::RandomTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_full_random_0() {
    let test_case = RandomTestCase::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_0",
    );
    test_case.execute();
}

#[test]
fn test_full_random_1() {
    let test_case = RandomTestCase::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_1",
    );
    test_case.execute();
}

#[test]
fn test_full_random_2() {
    let test_case = RandomTestCase::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_2",
    );
    test_case.execute();
}

#[test]
fn test_full_random_3() {
    let test_case = RandomTestCase::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_3",
    );
    test_case.execute();
}

#[test]
fn test_full_random_4() {
    let test_case = RandomTestCase::from(
        "consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_4",
    );
    test_case.execute();
}

#[test]
fn test_full_random_low_balances_0() {
    let  test_case = RandomTestCase::from("consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_low_balances_0");
    test_case.execute();
}

#[test]
fn test_full_random_low_balances_1() {
    let  test_case = RandomTestCase::from("consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_low_balances_1");
    test_case.execute();
}

#[test]
fn test_full_random_misc_balances() {
    let  test_case = RandomTestCase::from("consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_misc_balances");
    test_case.execute();
}

#[test]
fn test_full_random_without_leak_0() {
    let  test_case = RandomTestCase::from("consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_without_leak_0");
    test_case.execute();
}

#[test]
fn test_full_random_without_leak_and_current_exit_0() {
    let  test_case = RandomTestCase::from("consensus-spec-tests/tests/minimal/phase0/rewards/random/pyspec_tests/full_random_without_leak_and_current_exit_0");
    test_case.execute();
}
