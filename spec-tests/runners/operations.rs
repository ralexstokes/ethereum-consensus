use crate::{
    runners::{gen_exec, gen_match_for, gen_match_for_all},
    test_case::TestCase,
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use ethereum_consensus::{state_transition::Context, Error as SpecError};
use paste::paste;
use serde::Deserialize;

macro_rules! make_load_test {
    ($name: ident) => {
        paste! {
            fn [<load_ $name _test>] <S: ssz_rs::Deserialize, O: ssz_rs::Deserialize>(
                test_case_path: &str,
            ) -> (S, Option<S>, O) {
                let path = test_case_path.to_string() + "/pre.ssz_snappy";
                let pre: S = load_snappy_ssz(&path).unwrap();

                let path = test_case_path.to_string() + "/post.ssz_snappy";
                let post: Option<S> = load_snappy_ssz(&path);

                let path = test_case_path.to_string() + "/" + stringify!($name) + ".ssz_snappy";
                let operation: O = load_snappy_ssz(&path).unwrap();

                (pre, post, operation)
            }
        }
    };
    ($name: ident, $operation: ident) => {
        paste! {
            fn [<load_ $name _test>] <S: ssz_rs::Deserialize, O: ssz_rs::Deserialize>(
                test_case_path: &str,
            ) -> (S, Option<S>, O) {
                let path = test_case_path.to_string() + "/pre.ssz_snappy";
                let pre: S = load_snappy_ssz(&path).unwrap();

                let path = test_case_path.to_string() + "/post.ssz_snappy";
                let post: Option<S> = load_snappy_ssz(&path);

                let path = test_case_path.to_string() + "/" + stringify!($operation) + ".ssz_snappy";
                let operation: O = load_snappy_ssz(&path).unwrap();

                (pre, post, operation)
            }
        }
    };
}

make_load_test!(attestation);
make_load_test!(attester_slashing);
make_load_test!(block);
make_load_test!(deposit);
make_load_test!(proposer_slashing);
make_load_test!(voluntary_exit);
make_load_test!(sync_aggregate);
make_load_test!(withdrawals, execution_payload);
make_load_test!(bls_to_execution_change, address_change);

#[derive(Deserialize)]
struct ExecutionValidity {
    execution_valid: bool,
}

fn load_execution_payload_test<S: ssz_rs::Deserialize, O: ssz_rs::Deserialize>(
    test_case_path: &str,
) -> (S, Option<S>, O, bool) {
    let path = test_case_path.to_string() + "/pre.ssz_snappy";
    let pre: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/post.ssz_snappy";
    let post: Option<S> = load_snappy_ssz(&path);

    let path = test_case_path.to_string() + "/body.ssz_snappy";
    let operation: O = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/execution.yaml";
    let execution_validity: ExecutionValidity = load_yaml(&path);

    (pre, post, operation, execution_validity.execution_valid)
}

fn run_test<S: Eq, O, F>(
    mut pre: S,
    post: Option<S>,
    mut operation: O,
    context: &Context,
    exec_fn: F,
) -> Result<(), Error>
where
    F: FnOnce(&mut S, &mut O, &Context) -> Result<(), SpecError>,
{
    let operation = &mut operation;
    let result = exec_fn(&mut pre, operation, context);
    if let Some(post) = post {
        assert_eq!(result.unwrap(), ());
        if pre != post {
            Err(Error::InvalidState)
        } else {
            Ok(())
        }
    } else {
        if result.is_ok() {
            Err(Error::ExpectedError)
        } else {
            Ok(())
        }
    }
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "attestation" => {
            gen_match_for_all! {
                test,
                load_attestation_test,
                |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::Attestation), context| {
                    run_test(pre, post, operation, context, |state, operation, context| { spec::process_attestation(state, &*operation, context)} )
                }
            }
        }
        "attester_slashing" => {
            gen_match_for_all! {
                test,
                load_attester_slashing_test,
                |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::AttesterSlashing), context| {
                    run_test(pre, post, operation, context, spec::process_attester_slashing)
                }
            }
        }
        "block_header" => {
            gen_match_for_all! {
                test,
                load_block_test,
                |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::BeaconBlock), context| {
                    run_test(pre, post, operation, context, spec::process_block_header)
                }
            }
        }
        "deposit" => {
            gen_match_for_all! {
                test,
                load_deposit_test,
                |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::Deposit), context| {
                    run_test(pre, post, operation, context, spec::process_deposit)
                }
            }
        }
        "proposer_slashing" => {
            gen_match_for_all! {
                test,
                load_proposer_slashing_test,
                |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::ProposerSlashing), context| {
                    run_test(pre, post, operation, context, spec::process_proposer_slashing)
                }
            }
        }
        "voluntary_exit" => {
            gen_match_for_all! {
                test,
                load_voluntary_exit_test,
                |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SignedVoluntaryExit), context| {
                    run_test(pre, post, operation, context, spec::process_voluntary_exit)
                }
            }
        }
        "sync_aggregate" => {
            gen_match_for! {
                test,
                (mainnet, altair),
                (mainnet, bellatrix),
                (mainnet, capella),
                (mainnet, deneb),
                (minimal, altair),
                (minimal, bellatrix),
                (minimal, capella),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_sync_aggregate_test,
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SyncAggregate), context| {
                            run_test(pre, post, operation, context, |state, operation, context| { spec::process_sync_aggregate(state, &*operation, context)} )
                        }
                    }
                }
            }
        }
        "execution_payload" => {
            gen_match_for! {
                test,
                (mainnet, bellatrix),
                (mainnet, capella),
                (mainnet, deneb),
                (minimal, bellatrix),
                (minimal, capella),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_execution_payload_test,
                        |(pre, post, operation, execution_valid): (spec::BeaconState, Option<spec::BeaconState>, spec::BeaconBlockBody, bool), context: &Context| {
                            let mut context = context.clone();
                            context.execution_engine = execution_valid;
                            run_test(pre, post, operation, &context, spec::process_execution_payload)
                        }
                    }
                }
            }
        }
        "withdrawals" => {
            gen_match_for! {
                test,
                (mainnet, capella),
                (mainnet, deneb),
                (minimal, capella),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_withdrawals_test,
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::ExecutionPayload), context| {
                            run_test(pre, post, operation, context, |state, operation, context| { spec::process_withdrawals(state, &*operation, context)} )
                        }
                    }
                }
            }
        }
        "bls_to_execution_change" => {
            gen_match_for! {
                test,
                (mainnet, capella),
                (mainnet, deneb),
                (minimal, capella),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_bls_to_execution_change_test,
                        |(pre, post, operation): (spec::BeaconState, Option<spec::BeaconState>, spec::SignedBlsToExecutionChange), context| {
                            run_test(pre, post, operation, context, spec::process_bls_to_execution_change)
                        }
                    }
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
