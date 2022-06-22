// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::EffectiveBalanceUpdatesHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_effective_balance_hysteresis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/mainnet/altair/epoch_processing/effective_balance_updates/pyspec_tests/effective_balance_hysteresis");
    test_case.execute();
}
