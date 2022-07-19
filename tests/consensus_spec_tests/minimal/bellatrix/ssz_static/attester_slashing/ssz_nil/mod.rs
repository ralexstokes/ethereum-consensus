// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::AttesterSlashingTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_case_0() {
    let test_case = AttesterSlashingTestCase::from(
        "consensus-spec-tests/tests/minimal/bellatrix/ssz_static/AttesterSlashing/ssz_nil/case_0",
    );
    test_case.execute();
}
