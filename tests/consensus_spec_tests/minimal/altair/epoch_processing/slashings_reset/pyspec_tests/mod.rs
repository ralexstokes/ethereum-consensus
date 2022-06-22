// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::SlashingsResetHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_flush_slashings() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/slashings_reset/pyspec_tests/flush_slashings");
    test_case.execute();
}
