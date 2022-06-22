// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::BeaconStateHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_case_2() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/mainnet/altair/ssz_static/BeaconState/ssz_random/case_2",
    );
    test_case.execute();
}

#[test]
fn test_case_3() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/mainnet/altair/ssz_static/BeaconState/ssz_random/case_3",
    );
    test_case.execute();
}

#[test]
fn test_case_1() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/mainnet/altair/ssz_static/BeaconState/ssz_random/case_1",
    );
    test_case.execute();
}

#[test]
fn test_case_4() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/mainnet/altair/ssz_static/BeaconState/ssz_random/case_4",
    );
    test_case.execute();
}

#[test]
fn test_case_0() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/mainnet/altair/ssz_static/BeaconState/ssz_random/case_0",
    );
    test_case.execute();
}
