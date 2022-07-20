// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::ExecutionPayloadTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_bad_everything_regular_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_everything_regular_payload");

    test_case.execute();
}

#[test]
fn test_bad_execution_first_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_execution_first_payload");

    test_case.execute();
}

#[test]
fn test_bad_execution_regular_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_execution_regular_payload");

    test_case.execute();
}

#[test]
fn test_bad_parent_hash_regular_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_parent_hash_regular_payload");

    test_case.execute();
}

#[test]
fn test_bad_random_first_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_random_first_payload");

    test_case.execute();
}

#[test]
fn test_bad_random_regular_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_random_regular_payload");

    test_case.execute();
}

#[test]
fn test_bad_timestamp_first_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_timestamp_first_payload");

    test_case.execute();
}

#[test]
fn test_bad_timestamp_regular_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_timestamp_regular_payload");

    test_case.execute();
}

#[test]
fn test_non_empty_extra_data_first_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/non_empty_extra_data_first_payload");

    test_case.execute();
}

#[test]
fn test_non_empty_extra_data_regular_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/non_empty_extra_data_regular_payload");

    test_case.execute();
}

#[test]
fn test_success_first_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/success_first_payload");

    test_case.execute();
}

#[test]
fn test_success_first_payload_with_gap_slot() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/success_first_payload_with_gap_slot");

    test_case.execute();
}

#[test]
fn test_success_regular_payload() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/success_regular_payload");

    test_case.execute();
}

#[test]
fn test_success_regular_payload_with_gap_slot() {
    let  test_case = ExecutionPayloadTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/success_regular_payload_with_gap_slot");

    test_case.execute();
}
