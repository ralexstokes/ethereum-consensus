// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::SlashingsResetTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_flush_slashings() {
    let  test_case = SlashingsResetTestCase::from("consensus-spec-tests/tests/mainnet/phase0/epoch_processing/slashings_reset/pyspec_tests/flush_slashings");
    test_case.execute();
}
