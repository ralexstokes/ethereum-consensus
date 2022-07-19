// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::finality::FinalityTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_finality_no_updates_at_genesis() {
    let  test_case = FinalityTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/finality/finality/pyspec_tests/finality_no_updates_at_genesis");
    test_case.execute();
}

#[test]
fn test_finality_rule_1() {
    let  test_case = FinalityTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/finality/finality/pyspec_tests/finality_rule_1");
    test_case.execute();
}

#[test]
fn test_finality_rule_2() {
    let  test_case = FinalityTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/finality/finality/pyspec_tests/finality_rule_2");
    test_case.execute();
}

#[test]
fn test_finality_rule_3() {
    let  test_case = FinalityTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/finality/finality/pyspec_tests/finality_rule_3");
    test_case.execute();
}

#[test]
fn test_finality_rule_4() {
    let  test_case = FinalityTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/finality/finality/pyspec_tests/finality_rule_4");
    test_case.execute();
}
