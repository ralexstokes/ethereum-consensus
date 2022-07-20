// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::Eth1DataResetTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_eth_1_vote_no_reset() {
    let  test_case = Eth1DataResetTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/eth1_data_reset/pyspec_tests/eth1_vote_no_reset");

    test_case.execute();
}

#[test]
fn test_eth_1_vote_reset() {
    let  test_case = Eth1DataResetTestCase::from("consensus-spec-tests/tests/minimal/bellatrix/epoch_processing/eth1_data_reset/pyspec_tests/eth1_vote_reset");

    test_case.execute();
}
