// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::sanity::BlocksHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_prev_slot_block_transition() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/prev_slot_block_transition");
    test_case.execute();
}

#[test]
fn test_eth_1_data_votes_consensus() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/eth1_data_votes_consensus");
    test_case.execute();
}

#[test]
fn test_double_validator_exit_same_block() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/double_validator_exit_same_block");
    test_case.execute();
}

#[test]
fn test_skipped_slots() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/skipped_slots",
    );
    test_case.execute();
}

#[test]
fn test_empty_sync_committee_committee_genesis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/empty_sync_committee_committee_genesis");
    test_case.execute();
}

#[test]
fn test_empty_block_transition() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/empty_block_transition");
    test_case.execute();
}

#[test]
fn test_full_random_operations_2() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/full_random_operations_2");
    test_case.execute();
}

#[test]
fn test_high_proposer_index() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/high_proposer_index",
    );
    test_case.execute();
}

#[test]
fn test_attestation() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/attestation",
    );
    test_case.execute();
}

#[test]
fn test_invalid_proposer_index_sig_from_expected_proposer() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/invalid_proposer_index_sig_from_expected_proposer");
    test_case.execute();
}

#[test]
fn test_parent_from_same_slot() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/parent_from_same_slot");
    test_case.execute();
}

#[test]
fn test_deposit_in_block() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/deposit_in_block",
    );
    test_case.execute();
}

#[test]
fn test_proposer_slashing() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/proposer_slashing",
    );
    test_case.execute();
}

#[test]
fn test_double_similar_proposer_slashings_same_block() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/double_similar_proposer_slashings_same_block");
    test_case.execute();
}

#[test]
fn test_full_random_operations_0() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/full_random_operations_0");
    test_case.execute();
}

#[test]
fn test_inactivity_scores_full_participation_leaking() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/inactivity_scores_full_participation_leaking");
    test_case.execute();
}

#[test]
fn test_proposer_after_inactive_index() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/proposer_after_inactive_index");
    test_case.execute();
}

#[test]
fn test_empty_epoch_transition_not_finalizing() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/empty_epoch_transition_not_finalizing");
    test_case.execute();
}

#[test]
fn test_voluntary_exit() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/voluntary_exit",
    );
    test_case.execute();
}

#[test]
fn test_double_same_proposer_slashings_same_block() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/double_same_proposer_slashings_same_block");
    test_case.execute();
}

#[test]
fn test_empty_epoch_transition() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/empty_epoch_transition");
    test_case.execute();
}

#[test]
fn test_half_sync_committee_committee_genesis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/half_sync_committee_committee_genesis");
    test_case.execute();
}

#[test]
fn test_inactivity_scores_leaking() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/inactivity_scores_leaking");
    test_case.execute();
}

#[test]
fn test_empty_block_transition_large_validator_set() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/empty_block_transition_large_validator_set");
    test_case.execute();
}

#[test]
fn test_expected_deposit_in_block() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/expected_deposit_in_block");
    test_case.execute();
}

#[test]
fn test_full_random_operations_3() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/full_random_operations_3");
    test_case.execute();
}

#[test]
fn test_empty_sync_committee_committee() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/empty_sync_committee_committee");
    test_case.execute();
}

#[test]
fn test_multiple_attester_slashings_no_overlap() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/multiple_attester_slashings_no_overlap");
    test_case.execute();
}

#[test]
fn test_multiple_attester_slashings_partial_overlap() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/multiple_attester_slashings_partial_overlap");
    test_case.execute();
}

#[test]
fn test_slash_and_exit_diff_index() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/slash_and_exit_diff_index");
    test_case.execute();
}

#[test]
fn test_proposer_self_slashing() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/proposer_self_slashing");
    test_case.execute();
}

#[test]
fn test_half_sync_committee_committee() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/half_sync_committee_committee");
    test_case.execute();
}

#[test]
fn test_empty_epoch_transition_large_validator_set() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/empty_epoch_transition_large_validator_set");
    test_case.execute();
}

#[test]
fn test_attester_slashing() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/attester_slashing",
    );
    test_case.execute();
}

#[test]
fn test_same_slot_block_transition() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/same_slot_block_transition");
    test_case.execute();
}

#[test]
fn test_full_sync_committee_committee_genesis() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/full_sync_committee_committee_genesis");
    test_case.execute();
}

#[test]
fn test_full_sync_committee_committee() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/full_sync_committee_committee");
    test_case.execute();
}

#[test]
fn test_invalid_proposer_index_sig_from_proposer_index() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/invalid_proposer_index_sig_from_proposer_index");
    test_case.execute();
}

#[test]
fn test_full_random_operations_1() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/full_random_operations_1");
    test_case.execute();
}

#[test]
fn test_historical_batch() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/historical_batch",
    );
    test_case.execute();
}

#[test]
fn test_multiple_different_validator_exits_same_block() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/multiple_different_validator_exits_same_block");
    test_case.execute();
}

#[test]
fn test_zero_block_sig() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/zero_block_sig",
    );
    test_case.execute();
}

#[test]
fn test_multiple_different_proposer_slashings_same_block() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/multiple_different_proposer_slashings_same_block");
    test_case.execute();
}

#[test]
fn test_eth_1_data_votes_no_consensus() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/eth1_data_votes_no_consensus");
    test_case.execute();
}

#[test]
fn test_duplicate_attester_slashing() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/duplicate_attester_slashing");
    test_case.execute();
}

#[test]
fn test_slash_and_exit_same_index() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/slash_and_exit_same_index");
    test_case.execute();
}

#[test]
fn test_invalid_block_sig() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/invalid_block_sig",
    );
    test_case.execute();
}

#[test]
fn test_balance_driven_status_transitions() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/balance_driven_status_transitions");
    test_case.execute();
}

#[test]
fn test_deposit_top_up() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/deposit_top_up",
    );
    test_case.execute();
}

#[test]
fn test_invalid_state_root() {
    let test_case = TestRunner::from(
        "consensus-spec-tests/tests/minimal/altair/sanity/blocks/pyspec_tests/invalid_state_root",
    );
    test_case.execute();
}
