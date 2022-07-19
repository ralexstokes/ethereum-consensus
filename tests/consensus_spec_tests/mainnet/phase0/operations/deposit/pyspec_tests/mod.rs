// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::DepositTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_bad_merkle_proof() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/bad_merkle_proof");
    test_case.execute();
}

#[test]
fn test_invalid_sig_new_deposit() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/invalid_sig_new_deposit");
    test_case.execute();
}

#[test]
fn test_invalid_sig_other_version() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/invalid_sig_other_version");
    test_case.execute();
}

#[test]
fn test_invalid_sig_top_up() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/invalid_sig_top_up");
    test_case.execute();
}

#[test]
fn test_invalid_withdrawal_credentials_top_up() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/invalid_withdrawal_credentials_top_up");
    test_case.execute();
}

#[test]
fn test_new_deposit_eth_1_withdrawal_credentials() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/new_deposit_eth1_withdrawal_credentials");
    test_case.execute();
}

#[test]
fn test_new_deposit_max() {
    let test_case = DepositTestCase::from(
        "consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/new_deposit_max",
    );
    test_case.execute();
}

#[test]
fn test_new_deposit_non_versioned_withdrawal_credentials() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/new_deposit_non_versioned_withdrawal_credentials");
    test_case.execute();
}

#[test]
fn test_new_deposit_over_max() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/new_deposit_over_max");
    test_case.execute();
}

#[test]
fn test_new_deposit_under_max() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/new_deposit_under_max");
    test_case.execute();
}

#[test]
fn test_success_top_up() {
    let test_case = DepositTestCase::from(
        "consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/success_top_up",
    );
    test_case.execute();
}

#[test]
fn test_valid_sig_but_forked_state() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/valid_sig_but_forked_state");
    test_case.execute();
}

#[test]
fn test_wrong_deposit_for_deposit_count() {
    let  test_case = DepositTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/deposit/pyspec_tests/wrong_deposit_for_deposit_count");
    test_case.execute();
}
