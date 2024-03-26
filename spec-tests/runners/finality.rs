use crate::{
    runners::{
        gen_exec, gen_match_for_all,
        utils::{load_blocks_test, run_blocks_test},
    },
    test_case::TestCase,
    test_utils::Error,
};
use ethereum_consensus::state_transition::Validation;

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "finality" => {
            gen_match_for_all! {
                test,
                load_blocks_test,
                |(pre, post, blocks): (spec::BeaconState, Option<spec::BeaconState>, Vec<spec::SignedBeaconBlock>), context| {
                    run_blocks_test(pre, post, blocks, context, |state, signed_block, context| { spec::state_transition(state, signed_block, Validation::Enabled, context) })
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
