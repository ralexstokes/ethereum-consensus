// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::transition::CoreTestCase;

#[test]
fn test_normal_transition() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/normal_transition");

    test_case.execute();
}

#[test]
fn test_sample_transition() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/sample_transition");

    test_case.execute();
}

#[test]
fn test_transition_missing_first_post_block() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_missing_first_post_block");

    test_case.execute();
}

#[test]
fn test_transition_missing_last_pre_fork_block() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_missing_last_pre_fork_block");

    test_case.execute();
}

#[test]
fn test_transition_only_blocks_post_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_only_blocks_post_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_activation_at_fork_epoch() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_activation_at_fork_epoch");

    test_case.execute();
}

#[test]
fn test_transition_with_attester_slashing_right_after_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_attester_slashing_right_after_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_attester_slashing_right_before_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_attester_slashing_right_before_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_deposit_right_after_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_deposit_right_after_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_deposit_right_before_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_deposit_right_before_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_finality() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_finality");

    test_case.execute();
}

#[test]
fn test_transition_with_leaking_at_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_leaking_at_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_leaking_pre_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_leaking_pre_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_no_attestations_until_after_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_no_attestations_until_after_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_non_empty_activation_queue() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_non_empty_activation_queue");

    test_case.execute();
}

#[test]
fn test_transition_with_one_fourth_exiting_validators_exit_at_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_one_fourth_exiting_validators_exit_at_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_proposer_slashing_right_after_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_proposer_slashing_right_after_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_proposer_slashing_right_before_fork() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_proposer_slashing_right_before_fork");

    test_case.execute();
}

#[test]
fn test_transition_with_random_half_participation() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_random_half_participation");

    test_case.execute();
}

#[test]
fn test_transition_with_random_three_quarters_participation() {
    let  test_case = CoreTestCase::from("consensus-spec-tests/tests/mainnet/bellatrix/transition/core/pyspec_tests/transition_with_random_three_quarters_participation");

    test_case.execute();
}
