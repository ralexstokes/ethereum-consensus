// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::BlockHeaderTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_invalid_multiple_blocks_single_slot() {
    let  test_case = BlockHeaderTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/invalid_multiple_blocks_single_slot");

    test_case.execute();
}

#[test]
fn test_invalid_parent_root() {
    let  test_case = BlockHeaderTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/invalid_parent_root");

    test_case.execute();
}

#[test]
fn test_invalid_proposer_index() {
    let  test_case = BlockHeaderTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/invalid_proposer_index");

    test_case.execute();
}

#[test]
fn test_invalid_slot_block_header() {
    let  test_case = BlockHeaderTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/invalid_slot_block_header");

    test_case.execute();
}

#[test]
fn test_proposer_slashed() {
    let  test_case = BlockHeaderTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/proposer_slashed");

    test_case.execute();
}

#[test]
fn test_success_block_header() {
    let  test_case = BlockHeaderTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/success_block_header");

    test_case.execute();
}
