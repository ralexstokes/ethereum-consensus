use crate::{
    runners::{
        gen_exec, gen_match_for,
        light_client::{load_test, run_test, Proof},
    },
    test_case::TestCase,
    test_utils::Error,
};

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    match test.meta.handler.0.as_str() {
        "single_merkle_proof" => {
            gen_match_for! {
                test,
                (mainnet, deneb),
                (minimal, deneb)
                {
                    gen_exec! {
                        test,
                        load_test,
                        |(object, proof): (spec::BeaconBlockBody, Proof), _| {
                            // NOTE: `0` index is hard-coded in test generator
                            let path = &["blob_kzg_commitments".into(), 0.into()];
                            run_test(object, path, &proof)
                        }
                    }
                }
            }
        }
        handler => unreachable!("no tests for {handler}"),
    }
}
