// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::sanity::SlotsHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_over_epoch_boundary() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/slots/pyspec_tests/over_epoch_boundary",
    );
    test_case.execute();
}

#[test]
fn test_double_empty_epoch() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/slots/pyspec_tests/double_empty_epoch",
    );
    test_case.execute();
}

#[test]
fn test_slots_2() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/slots/pyspec_tests/slots_2",
    );
    test_case.execute();
}

#[test]
fn test_slots_1() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/slots/pyspec_tests/slots_1",
    );
    test_case.execute();
}

#[test]
fn test_empty_epoch() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/slots/pyspec_tests/empty_epoch",
    );
    test_case.execute();
}
