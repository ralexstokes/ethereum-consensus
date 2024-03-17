use crate::{
    runners::{gen_dispatch, gen_exec},
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz_bytes, load_yaml, Error},
};
use ethereum_consensus::primitives::Bytes32;
use serde::Deserialize;
use ssz_rs::prelude::*;
use std::path::Path;

#[derive(Deserialize)]
struct RootData {
    root: Bytes32,
}

fn load_test(test_case_path: &str) -> (RootData, Vec<u8>) {
    let path = test_case_path.to_string() + "/roots.yaml";
    let data: RootData = load_yaml(&path);

    let path = test_case_path.to_string() + "/serialized.ssz_snappy";
    let path = Path::new(&path);
    let encoding = load_snappy_ssz_bytes(path);
    (data, encoding)
}

macro_rules! gen_test {
    ($handler:ident) => {
        |(data, encoding): (RootData, Vec<u8>), _| {
            let mut decoded_data: spec::$handler = deserialize(&encoding).unwrap();
            let serialized = serialize(&decoded_data).unwrap();
            let root = decoded_data.hash_tree_root().unwrap();
            assert_eq!(serialized, encoding);
            assert_eq!(root.as_ref(), data.root.as_ref());
            Ok(())
        }
    };
}

macro_rules! gen_match {
    ($meta:ident, $path:ident, $($handler:ident),*) => {
        match $meta.handler.0.as_str() {
            $(
                stringify!($handler) => {
                    gen_dispatch! {
                        $path,
                        $meta.config,
                        $meta.fork,
                        load_test,
                        gen_test!($handler)
                    }
                }
            )*
            handler => Err(Error::UnknownHandler(handler.into(), $meta.name())),
        }
    };
}

macro_rules! gen_bellatrix_and_later {
    ($meta:ident, $config:expr, $path:ident, $($handler:ident),*) => {
        let result = match $meta.handler.0.as_str() {
            $(
                stringify!($handler) => match $config {
                    Config::Mainnet => match $meta.fork {
                        Fork::Bellatrix => {
                            gen_exec! {
                                use ethereum_consensus::bellatrix::mainnet as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Capella => {
                            gen_exec! {
                                use ethereum_consensus::capella::mainnet as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Deneb => {
                            gen_exec! {
                                use ethereum_consensus::deneb::mainnet as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        _ => unreachable!(),
                    },
                    Config::Minimal => match $meta.fork {
                        Fork::Bellatrix => {
                            gen_exec! {
                                use ethereum_consensus::bellatrix::minimal as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Capella => {
                            gen_exec! {
                                use ethereum_consensus::capella::minimal as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Deneb => {
                            gen_exec! {
                                use ethereum_consensus::deneb::minimal as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
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
    ($meta:ident, $config:expr, $path:ident, $($handler:ident),*) => {
        let result = match $meta.handler.0.as_str() {
            $(
                stringify!($handler) => match $config {
                    Config::Mainnet => match $meta.fork {
                        Fork::Altair => {
                            gen_exec! {
                                use ethereum_consensus::altair::mainnet as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Bellatrix => {
                            gen_exec! {
                                use ethereum_consensus::bellatrix::mainnet as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Capella => {
                            gen_exec! {
                                use ethereum_consensus::capella::mainnet as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Deneb => {
                            gen_exec! {
                                use ethereum_consensus::deneb::mainnet as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        _ => unreachable!(),
                    },
                    Config::Minimal => match $meta.fork {
                        Fork::Altair => {
                            gen_exec! {
                                use ethereum_consensus::altair::minimal as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Bellatrix => {
                            gen_exec! {
                                use ethereum_consensus::bellatrix::minimal as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Capella => {
                            gen_exec! {
                                use ethereum_consensus::capella::minimal as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        Fork::Deneb => {
                            gen_exec! {
                                use ethereum_consensus::deneb::minimal as spec;
                                $path, $config, load_test, gen_test! { $handler }
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
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

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    let path = &test.data_path;

    gen_bellatrix_and_later! {
        meta,
        meta.config,
        path,
        ExecutionPayload,
        ExecutionPayloadHeader,
        PowBlock
    }

    gen_altair_and_later! {
        meta,
        meta.config,
        path,
        ContributionAndProof,
        LightClientUpdate,
        SignedContributionAndProof,
        SyncAggregate,
        SyncAggregatorSelectionData,
        SyncCommittee,
        SyncCommitteeContribution,
        SyncCommitteeMessage
    }

    gen_match! {
        meta,
        path,
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
