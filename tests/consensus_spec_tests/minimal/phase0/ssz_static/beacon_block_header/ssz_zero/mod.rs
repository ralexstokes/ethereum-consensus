// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::BeaconBlockHeaderTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_case_0() {
    let test_case = BeaconBlockHeaderTestCase::from(
        "consensus-spec-tests/tests/minimal/phase0/ssz_static/BeaconBlockHeader/ssz_zero/case_0",
    );
    test_case.execute();
}