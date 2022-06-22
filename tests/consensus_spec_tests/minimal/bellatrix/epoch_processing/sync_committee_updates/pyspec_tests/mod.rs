// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::SyncCommitteeUpdatesHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_sync_committees_progress_misc_balances_not_genesis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/sync_committee_updates/pyspec_tests/sync_committees_progress_misc_balances_not_genesis");
    test_case.execute();
}

#[test]
fn test_sync_committees_progress_misc_balances_genesis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/sync_committee_updates/pyspec_tests/sync_committees_progress_misc_balances_genesis");
    test_case.execute();
}

#[test]
fn test_sync_committees_progress_genesis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/sync_committee_updates/pyspec_tests/sync_committees_progress_genesis");
    test_case.execute();
}

#[test]
fn test_sync_committees_progress_not_genesis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/sync_committee_updates/pyspec_tests/sync_committees_progress_not_genesis");
    test_case.execute();
}

#[test]
fn test_sync_committees_no_progress_not_boundary() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/sync_committee_updates/pyspec_tests/sync_committees_no_progress_not_boundary");
    test_case.execute();
}
