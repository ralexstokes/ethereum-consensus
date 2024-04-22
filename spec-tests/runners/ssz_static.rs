use crate::{
    runners::{gen_exec, gen_match_for, gen_match_for_all},
    test_case::TestCase,
    test_utils::{load_snappy_ssz_bytes, load_yaml, Error},
};
use ethereum_consensus::{primitives::Root, state_transition::Context};
use serde::Deserialize;
use ssz_rs::prelude::*;
use std::path::Path;

#[derive(Deserialize)]
struct RootData {
    root: Root,
}

fn load_test(test_case_path: &str) -> (RootData, Vec<u8>) {
    let path = test_case_path.to_string() + "/roots.yaml";
    let data: RootData = load_yaml(&path);

    let path = test_case_path.to_string() + "/serialized.ssz_snappy";
    let path = Path::new(&path);
    let encoding = load_snappy_ssz_bytes(path);
    (data, encoding)
}

fn run_test<T: ssz_rs::SimpleSerialize>(
    (data, encoding): (RootData, Vec<u8>),
    _: &Context,
) -> Result<(), Error> {
    let decoded_data: T = deserialize(&encoding).unwrap();
    let serialized = serialize(&decoded_data).unwrap();
    let root = decoded_data.hash_tree_root().unwrap();
    assert_eq!(serialized, encoding);
    assert_eq!(root, data.root);
    Ok(())
}

macro_rules! gen_electra {
    ($test_case:expr, $($handler:ident),*) => {
        let result = match $test_case.meta.handler.0.as_str() {
            $(
                stringify!($handler) => gen_match_for! {
                    $test_case,
                    (mainnet, electra),
                    (minimal, electra)
                    {
                        gen_exec! {
                            $test_case, load_test, run_test::<spec::$handler>
                        }
                    }
                },
            )*
            _ => Err(Error::InternalContinue),
        };
        match result {
            Ok(()) => return Ok(()),
            Err(Error::InternalContinue) => {},
            Err(err) => return Err(err)
        }
    };
}

macro_rules! gen_deneb_and_later {
    ($test_case:expr, $($handler:ident),*) => {
        let result = match $test_case.meta.handler.0.as_str() {
            $(
                stringify!($handler) => gen_match_for! {
                    $test_case,
                    (mainnet, deneb),
                    (minimal, deneb)
                    {
                        gen_exec! {
                            $test_case, load_test, run_test::<spec::$handler>
                        }
                    }
                },
            )*
            _ => Err(Error::InternalContinue),
        };
        match result {
            Ok(()) => return Ok(()),
            Err(Error::InternalContinue) => {},
            Err(err) => return Err(err)
        }
    };
}

macro_rules! gen_capella_and_later {
    ($test_case:expr, $($handler:ident),*) => {
        let result = match $test_case.meta.handler.0.as_str() {
            $(
                stringify!($handler) => gen_match_for! {
                    $test_case,
                    (mainnet, capella),
                    (mainnet, deneb),
                    (minimal, capella),
                    (minimal, deneb)
                    {
                        gen_exec! {
                            $test_case, load_test, run_test::<spec::$handler>
                        }
                    }
                },
            )*
            _ => Err(Error::InternalContinue),
        };
        match result {
            Ok(()) => return Ok(()),
            Err(Error::InternalContinue) => {},
            Err(err) => return Err(err)
        }
    };
}

macro_rules! gen_bellatrix_and_later {
    ($test_case:expr, $($handler:ident),*) => {
        let result = match $test_case.meta.handler.0.as_str() {
            $(
                stringify!($handler) => gen_match_for! {
                    $test_case,
                    (mainnet, bellatrix),
                    (mainnet, capella),
                    (mainnet, deneb),
                    (minimal, bellatrix),
                    (minimal, capella),
                    (minimal, deneb)
                    {
                        gen_exec! {
                            $test_case, load_test, run_test::<spec::$handler>
                        }
                    }
                },
            )*
            _ => Err(Error::InternalContinue),
        };
        match result {
            Ok(()) => return Ok(()),
            Err(Error::InternalContinue) => {},
            Err(err) => return Err(err)
        }
    };
}

macro_rules! gen_altair_and_later {
    ($test_case:expr, $($handler:ident),*) => {
        let result = match $test_case.meta.handler.0.as_str() {
            $(
                stringify!($handler) => gen_match_for! {
                    $test_case,
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
                            $test_case, load_test, run_test::<spec::$handler>
                        }
                    }
                },
            )*
            _ => Err(Error::InternalContinue),
        };
        match result {
            Ok(()) => return Ok(()),
            Err(Error::InternalContinue) => {},
            Err(err) => return Err(err)
        }
    };
}

macro_rules! gen_match {
    ($test_case:expr, $($handler:ident),*) => {
        match $test_case.meta.handler.0.as_str() {
            $(
                stringify!($handler) => {
                    gen_match_for_all! {
                        $test_case,
                        load_test,
                        run_test::<spec::$handler>
                    }
                }
            )*
            handler => unreachable!("no tests for {handler}"),
        }
    };
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    gen_electra! {
        test,
        Attestation
    }

    gen_deneb_and_later! {
        test,
        BlobSidecar,
        BlobIdentifier
    }

    gen_capella_and_later! {
        test,
        Withdrawal,
        HistoricalSummary,
        BlsToExecutionChange,
        SignedBlsToExecutionChange
    }

    gen_bellatrix_and_later! {
        test,
        ExecutionPayload,
        ExecutionPayloadHeader,
        PowBlock
    }

    gen_altair_and_later! {
        test,
        ContributionAndProof,
        LightClientHeader,
        LightClientBootstrap,
        LightClientUpdate,
        LightClientFinalityUpdate,
        LightClientOptimisticUpdate,
        SignedContributionAndProof,
        SyncAggregate,
        SyncAggregatorSelectionData,
        SyncCommittee,
        SyncCommitteeContribution,
        SyncCommitteeMessage
    }

    gen_match! {
        test,
        AggregateAndProof,
        Attestation,
        AttestationData,
        AttesterSlashing,
        BeaconBlock,
        BeaconBlockBody,
        BeaconBlockHeader,
        BeaconState,
        Checkpoint,
        Deposit,
        DepositData,
        DepositMessage,
        Eth1Block,
        Eth1Data,
        Fork,
        ForkData,
        HistoricalBatch,
        IndexedAttestation,
        PendingAttestation,
        ProposerSlashing,
        SignedAggregateAndProof,
        SignedBeaconBlock,
        SignedBeaconBlockHeader,
        SignedVoluntaryExit,
        SigningData,
        Validator,
        VoluntaryExit
    }
}
