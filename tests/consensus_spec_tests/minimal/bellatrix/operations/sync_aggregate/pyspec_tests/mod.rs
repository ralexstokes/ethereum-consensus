// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::SyncAggregateHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_sync_committee_with_participating_withdrawable_member() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/sync_committee_with_participating_withdrawable_member");
    test_case.execute();
}

#[test]
fn test_random_all_but_one_participating_without_duplicates() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/random_all_but_one_participating_without_duplicates");
    test_case.execute();
}

#[test]
fn test_random_high_participation_without_duplicates() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/random_high_participation_without_duplicates");
    test_case.execute();
}

#[test]
fn test_proposer_in_committee_with_participation() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/proposer_in_committee_with_participation");
    test_case.execute();
}

#[test]
fn test_invalid_signature_previous_committee() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/invalid_signature_previous_committee");
    test_case.execute();
}

#[test]
fn test_invalid_signature_infinite_signature_with_single_participant() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/invalid_signature_infinite_signature_with_single_participant");
    test_case.execute();
}

#[test]
fn test_sync_committee_with_nonparticipating_withdrawable_member() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/sync_committee_with_nonparticipating_withdrawable_member");
    test_case.execute();
}

#[test]
fn test_random_only_one_participant_without_duplicates() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/random_only_one_participant_without_duplicates");
    test_case.execute();
}

#[test]
fn test_invalid_signature_no_participants() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/invalid_signature_no_participants");
    test_case.execute();
}

#[test]
fn test_random_low_participation_without_duplicates() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/random_low_participation_without_duplicates");
    test_case.execute();
}

#[test]
fn test_proposer_in_committee_without_participation() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/proposer_in_committee_without_participation");
    test_case.execute();
}

#[test]
fn test_invalid_signature_bad_domain() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/invalid_signature_bad_domain");
    test_case.execute();
}

#[test]
fn test_invalid_signature_past_block() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/invalid_signature_past_block");
    test_case.execute();
}

#[test]
fn test_sync_committee_rewards_empty_participants() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/sync_committee_rewards_empty_participants");
    test_case.execute();
}

#[test]
fn test_invalid_signature_extra_participant() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/invalid_signature_extra_participant");
    test_case.execute();
}

#[test]
fn test_invalid_signature_infinite_signature_with_all_participants() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/invalid_signature_infinite_signature_with_all_participants");
    test_case.execute();
}

#[test]
fn test_sync_committee_rewards_not_full_participants() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/sync_committee_rewards_not_full_participants");
    test_case.execute();
}

#[test]
fn test_sync_committee_rewards_nonduplicate_committee() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/sync_committee_rewards_nonduplicate_committee");
    test_case.execute();
}

#[test]
fn test_sync_committee_with_nonparticipating_exited_member() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/sync_committee_with_nonparticipating_exited_member");
    test_case.execute();
}

#[test]
fn test_random_with_exits_without_duplicates() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/random_with_exits_without_duplicates");
    test_case.execute();
}

#[test]
fn test_random_misc_balances_and_half_participation_without_duplicates() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/random_misc_balances_and_half_participation_without_duplicates");
    test_case.execute();
}

#[test]
fn test_valid_signature_future_committee() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/valid_signature_future_committee");
    test_case.execute();
}

#[test]
fn test_invalid_signature_missing_participant() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/invalid_signature_missing_participant");
    test_case.execute();
}

#[test]
fn test_sync_committee_with_participating_exited_member() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/operations/sync_aggregate/pyspec_tests/sync_committee_with_participating_exited_member");
    test_case.execute();
}
