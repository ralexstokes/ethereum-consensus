// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::RewardsAndPenaltiesHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_full_attestations_misc_balances() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/full_attestations_misc_balances");
    test_case.execute();
}

#[test]
fn test_random_fill_attestations_with_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/random_fill_attestations_with_leak");
    test_case.execute();
}

#[test]
fn test_full_attestation_participation_with_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/full_attestation_participation_with_leak");
    test_case.execute();
}

#[test]
fn test_full_attestations_one_validaor_one_gwei() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/full_attestations_one_validaor_one_gwei");
    test_case.execute();
}

#[test]
fn test_duplicate_attestation() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/duplicate_attestation");
    test_case.execute();
}

#[test]
fn test_full_attestation_participation() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/full_attestation_participation");
    test_case.execute();
}

#[test]
fn test_almost_full_attestations() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/almost_full_attestations");
    test_case.execute();
}

#[test]
fn test_almost_empty_attestations_with_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/almost_empty_attestations_with_leak");
    test_case.execute();
}

#[test]
fn test_no_attestations_all_penalties() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/no_attestations_all_penalties");
    test_case.execute();
}

#[test]
fn test_random_fill_attestations() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/random_fill_attestations");
    test_case.execute();
}

#[test]
fn test_almost_full_attestations_with_leak() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/almost_full_attestations_with_leak");
    test_case.execute();
}

#[test]
fn test_genesis_epoch_full_attestations_no_rewards() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/genesis_epoch_full_attestations_no_rewards");
    test_case.execute();
}

#[test]
fn test_attestations_some_slashed() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/attestations_some_slashed");
    test_case.execute();
}

#[test]
fn test_almost_empty_attestations() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/almost_empty_attestations");
    test_case.execute();
}

#[test]
fn test_genesis_epoch_no_attestations_no_penalties() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/rewards_and_penalties/pyspec_tests/genesis_epoch_no_attestations_no_penalties");
    test_case.execute();
}
