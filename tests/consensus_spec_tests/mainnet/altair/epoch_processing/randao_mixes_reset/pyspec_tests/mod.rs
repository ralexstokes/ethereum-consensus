// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::RandaoMixesResetTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_updated_randao_mixes() {
    let  test_case = RandaoMixesResetTestCase::from("consensus-spec-tests/tests/mainnet/altair/epoch_processing/randao_mixes_reset/pyspec_tests/updated_randao_mixes");

    test_case.execute();
}
