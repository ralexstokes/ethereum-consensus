// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::ParticipationFlagUpdatesHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_random_2() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/random_2");
    test_case.execute();
}

#[test]
fn test_random_1() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/random_1");
    test_case.execute();
}

#[test]
fn test_current_epoch_zeroed() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/current_epoch_zeroed");
    test_case.execute();
}

#[test]
fn test_slightly_larger_random() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/slightly_larger_random");
    test_case.execute();
}

#[test]
fn test_filled() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/filled");
    test_case.execute();
}

#[test]
fn test_previous_epoch_zeroed() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/previous_epoch_zeroed");
    test_case.execute();
}

#[test]
fn test_all_zeroed() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/all_zeroed");
    test_case.execute();
}

#[test]
fn test_random_0() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/random_0");
    test_case.execute();
}

#[test]
fn test_random_genesis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/random_genesis");
    test_case.execute();
}

#[test]
fn test_large_random() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/large_random");
    test_case.execute();
}

#[test]
fn test_current_filled() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/current_filled");
    test_case.execute();
}

#[test]
fn test_previous_filled() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/participation_flag_updates/pyspec_tests/previous_filled");
    test_case.execute();
}
