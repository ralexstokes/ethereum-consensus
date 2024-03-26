use crate::{
    runners::{
        gen_exec, gen_match_for_all,
        utils::{load_blocks_test, run_blocks_test},
    },
    test_case::TestCase,
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use ethereum_consensus::state_transition::Validation;

fn load_test<S: ssz_rs::Deserialize>(test_case_path: &str) -> (S, S, u64) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/slots.yaml";
    let slots: u64 = load_yaml(&path);

    (pre, post, slots)
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "blocks" => {
            gen_match_for_all! {
                test,
                load_blocks_test,
                |(pre, post, blocks): (spec::BeaconState, Option<spec::BeaconState>, Vec<spec::SignedBeaconBlock>), context| {
                    run_blocks_test(pre, post, blocks, context, |state, signed_block, context| { spec::state_transition(state, signed_block, Validation::Enabled, context) })
                }
            }
        }
        "slots" => {
            gen_match_for_all! {
                test,
                load_test,
                |(mut pre, post, slots): (spec::BeaconState, spec::BeaconState, u64), context| {
                    let target_slot = pre.slot + slots;
                    spec::process_slots(&mut pre, target_slot, context)?;
                    if pre != post {
                        Err(Error::InvalidState)
                    } else {
                        Ok(())
                    }
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
