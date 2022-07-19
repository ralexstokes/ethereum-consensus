// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::genesis::InitializationTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_initialize_beacon_state_from_eth_1() {
    let  test_case = InitializationTestCase::from("consensus-spec-tests/tests/minimal/altair/genesis/initialization/pyspec_tests/initialize_beacon_state_from_eth1");
    test_case.execute();
}

#[test]
fn test_initialize_beacon_state_one_topup_activation() {
    let  test_case = InitializationTestCase::from("consensus-spec-tests/tests/minimal/altair/genesis/initialization/pyspec_tests/initialize_beacon_state_one_topup_activation");
    test_case.execute();
}

#[test]
fn test_initialize_beacon_state_random_invalid_genesis() {
    let  test_case = InitializationTestCase::from("consensus-spec-tests/tests/minimal/altair/genesis/initialization/pyspec_tests/initialize_beacon_state_random_invalid_genesis");
    test_case.execute();
}

#[test]
fn test_initialize_beacon_state_random_valid_genesis() {
    let  test_case = InitializationTestCase::from("consensus-spec-tests/tests/minimal/altair/genesis/initialization/pyspec_tests/initialize_beacon_state_random_valid_genesis");
    test_case.execute();
}

#[test]
fn test_initialize_beacon_state_some_small_balances() {
    let  test_case = InitializationTestCase::from("consensus-spec-tests/tests/minimal/altair/genesis/initialization/pyspec_tests/initialize_beacon_state_some_small_balances");
    test_case.execute();
}
