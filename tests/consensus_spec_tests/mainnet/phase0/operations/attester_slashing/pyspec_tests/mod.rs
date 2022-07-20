// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::AttesterSlashingTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_all_empty_indices() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/all_empty_indices");

    test_case.execute();
}

#[test]
fn test_att_1_bad_extra_index() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att1_bad_extra_index");

    test_case.execute();
}

#[test]
fn test_att_1_bad_replaced_index() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att1_bad_replaced_index");

    test_case.execute();
}

#[test]
fn test_att_1_duplicate_index_double_signed() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att1_duplicate_index_double_signed");

    test_case.execute();
}

#[test]
fn test_att_1_duplicate_index_normal_signed() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att1_duplicate_index_normal_signed");

    test_case.execute();
}

#[test]
fn test_att_1_empty_indices() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att1_empty_indices");

    test_case.execute();
}

#[test]
fn test_att_1_high_index() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att1_high_index");

    test_case.execute();
}

#[test]
fn test_att_2_bad_extra_index() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att2_bad_extra_index");

    test_case.execute();
}

#[test]
fn test_att_2_bad_replaced_index() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att2_bad_replaced_index");

    test_case.execute();
}

#[test]
fn test_att_2_duplicate_index_double_signed() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att2_duplicate_index_double_signed");

    test_case.execute();
}

#[test]
fn test_att_2_duplicate_index_normal_signed() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att2_duplicate_index_normal_signed");

    test_case.execute();
}

#[test]
fn test_att_2_empty_indices() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att2_empty_indices");

    test_case.execute();
}

#[test]
fn test_att_2_high_index() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/att2_high_index");

    test_case.execute();
}

#[test]
fn test_invalid_sig_1() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/invalid_sig_1");

    test_case.execute();
}

#[test]
fn test_invalid_sig_1_and_2() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/invalid_sig_1_and_2");

    test_case.execute();
}

#[test]
fn test_invalid_sig_2() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/invalid_sig_2");

    test_case.execute();
}

#[test]
fn test_no_double_or_surround() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/no_double_or_surround");

    test_case.execute();
}

#[test]
fn test_participants_already_slashed() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/participants_already_slashed");

    test_case.execute();
}

#[test]
fn test_same_data() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/same_data");

    test_case.execute();
}

#[test]
fn test_success_already_exited_long_ago() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_already_exited_long_ago");

    test_case.execute();
}

#[test]
fn test_success_already_exited_recent() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_already_exited_recent");

    test_case.execute();
}

#[test]
fn test_success_attestation_from_future() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_attestation_from_future");

    test_case.execute();
}

#[test]
fn test_success_double() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_double");

    test_case.execute();
}

#[test]
fn test_success_low_balances() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_low_balances");

    test_case.execute();
}

#[test]
fn test_success_misc_balances() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_misc_balances");

    test_case.execute();
}

#[test]
fn test_success_proposer_index_slashed() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_proposer_index_slashed");

    test_case.execute();
}

#[test]
fn test_success_surround() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_surround");

    test_case.execute();
}

#[test]
fn test_success_with_effective_balance_disparity() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/success_with_effective_balance_disparity");

    test_case.execute();
}

#[test]
fn test_unsorted_att_1() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/unsorted_att_1");

    test_case.execute();
}

#[test]
fn test_unsorted_att_2() {
    let  test_case = AttesterSlashingTestCase::from("consensus-spec-tests/tests/mainnet/phase0/operations/attester_slashing/pyspec_tests/unsorted_att_2");

    test_case.execute();
}
