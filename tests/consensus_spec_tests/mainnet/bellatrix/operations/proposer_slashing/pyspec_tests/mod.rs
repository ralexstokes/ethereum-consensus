// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::ProposerSlashingTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_epochs_are_different() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/epochs_are_different");
    test_case.execute();
}

#[test]
fn test_headers_are_same_sigs_are_different() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/headers_are_same_sigs_are_different");
    test_case.execute();
}

#[test]
fn test_headers_are_same_sigs_are_same() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/headers_are_same_sigs_are_same");
    test_case.execute();
}

#[test]
fn test_invalid_different_proposer_indices() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/invalid_different_proposer_indices");
    test_case.execute();
}

#[test]
fn test_invalid_proposer_index() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/invalid_proposer_index");
    test_case.execute();
}

#[test]
fn test_invalid_sig_1() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/invalid_sig_1");
    test_case.execute();
}

#[test]
fn test_invalid_sig_1_and_2() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/invalid_sig_1_and_2");
    test_case.execute();
}

#[test]
fn test_invalid_sig_1_and_2_swap() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/invalid_sig_1_and_2_swap");
    test_case.execute();
}

#[test]
fn test_invalid_sig_2() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/invalid_sig_2");
    test_case.execute();
}

#[test]
fn test_proposer_is_not_activated() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/proposer_is_not_activated");
    test_case.execute();
}

#[test]
fn test_proposer_is_slashed() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/proposer_is_slashed");
    test_case.execute();
}

#[test]
fn test_proposer_is_withdrawn() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/proposer_is_withdrawn");
    test_case.execute();
}

#[test]
fn test_success() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/success");
    test_case.execute();
}

#[test]
fn test_success_block_header_from_future() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/success_block_header_from_future");
    test_case.execute();
}

#[test]
fn test_success_slashed_and_proposer_index_the_same() {
    let  test_case = ProposerSlashingTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/proposer_slashing/pyspec_tests/success_slashed_and_proposer_index_the_same");
    test_case.execute();
}
