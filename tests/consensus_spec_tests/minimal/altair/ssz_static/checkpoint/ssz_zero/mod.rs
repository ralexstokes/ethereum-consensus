// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::CheckpointTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_case_0() {
    let test_case = CheckpointTestCase::from(
        "consensus-spec-tests/tests/minimal/altair/ssz_static/Checkpoint/ssz_zero/case_0",
    );

    test_case.execute();
}
