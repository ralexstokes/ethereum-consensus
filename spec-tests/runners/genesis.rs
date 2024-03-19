use crate::{
    runners::gen_exec,
    test_case::TestCase,
    test_meta::{Config, Fork},
    test_utils::{load_snappy_ssz, load_yaml, Error},
};
use ethereum_consensus::primitives::Bytes32;
use serde::Deserialize;

#[derive(Deserialize)]
struct Eth1 {
    eth1_block_hash: Bytes32,
    eth1_timestamp: u64,
}

#[derive(Deserialize)]
struct Meta {
    deposits_count: usize,
    execution_payload_header: Option<bool>,
}

fn load_initialization_test<
    S: ssz_rs::Deserialize,
    D: ssz_rs::Deserialize,
    H: ssz_rs::Deserialize,
>(
    test_case_path: &str,
) -> (Eth1, Vec<D>, Option<H>, S) {
    let path = test_case_path.to_string() + "/eth1.yaml";
    let eth1: Eth1 = load_yaml(&path);

    let path = test_case_path.to_string() + "/meta.yaml";
    let meta: Meta = load_yaml(&path);

    let mut deposits = vec![];
    for i in 0..meta.deposits_count {
        let path = format!("{test_case_path}/deposits_{i}.ssz_snappy");
        let deposit: D = load_snappy_ssz(&path).unwrap();
        deposits.push(deposit);
    }

    let execution_payload_header = if meta.execution_payload_header.unwrap_or(false) {
        let path = test_case_path.to_string() + "/execution_payload_header.ssz_snappy";
        let header: H = load_snappy_ssz(&path).unwrap();
        Some(header)
    } else {
        None
    };

    let path = test_case_path.to_string() + "/state.ssz_snappy";
    let state: S = load_snappy_ssz(&path).unwrap();

    (eth1, deposits, execution_payload_header, state)
}

fn load_validity_test<S: ssz_rs::Deserialize>(test_case_path: &str) -> (S, bool) {
    let path = test_case_path.to_string() + "/genesis.ssz_snappy";
    let state: S = load_snappy_ssz(&path).unwrap();

    let path = test_case_path.to_string() + "/is_valid.yaml";
    let is_valid: bool = load_yaml(&path);

    (state, is_valid)
}

pub fn dispatch(test: &TestCase) -> Result<(), Error> {
    let meta = &test.meta;
    let path = &test.data_path;
    match meta.handler.0.as_str() {
        "initialization" => match meta.config {
            Config::Minimal => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::minimal as spec;
                        path,
                        meta.config,
                        |path| {
                            load_initialization_test::<
                                spec::BeaconState,
                                spec::Deposit,
                                bool, // dummy type
                            >(path)
                        },
                        |(eth1, mut deposits, _, expected): (
                            Eth1,
                            Vec<spec::Deposit>,
                            Option<bool>,
                            spec::BeaconState
                        ),
                         context| {
                            let state = spec::initialize_beacon_state_from_eth1::<
                                { spec::SLOTS_PER_HISTORICAL_ROOT },
                                { spec::HISTORICAL_ROOTS_LIMIT },
                                { spec::ETH1_DATA_VOTES_BOUND },
                                { spec::VALIDATOR_REGISTRY_LIMIT },
                                { spec::EPOCHS_PER_HISTORICAL_VECTOR },
                                { spec::EPOCHS_PER_SLASHINGS_VECTOR },
                                { spec::MAX_VALIDATORS_PER_COMMITTEE },
                                { spec::PENDING_ATTESTATIONS_BOUND },
                                { spec::MAX_PROPOSER_SLASHINGS },
                                { spec::MAX_ATTESTER_SLASHINGS },
                                { spec::MAX_ATTESTATIONS },
                                { spec::MAX_DEPOSITS },
                                { spec::MAX_VOLUNTARY_EXITS },
                            >(
                                eth1.eth1_block_hash,
                                eth1.eth1_timestamp,
                                &mut deposits,
                                context,
                            )
                            .unwrap();
                            if expected == state {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::minimal as spec;
                        path,
                        meta.config,
                        |path| {
                            load_initialization_test::<
                                spec::BeaconState,
                                spec::Deposit,
                                bool, // dummy type
                            >(path)
                        },
                        |(eth1, mut deposits, _, expected): (
                            Eth1,
                            Vec<spec::Deposit>,
                            Option<bool>,
                            spec::BeaconState
                        ),
                         context| {
                            let state = spec::initialize_beacon_state_from_eth1::<
                                { spec::SLOTS_PER_HISTORICAL_ROOT },
                                { spec::HISTORICAL_ROOTS_LIMIT },
                                { spec::ETH1_DATA_VOTES_BOUND },
                                { spec::VALIDATOR_REGISTRY_LIMIT },
                                { spec::EPOCHS_PER_HISTORICAL_VECTOR },
                                { spec::EPOCHS_PER_SLASHINGS_VECTOR },
                                { spec::MAX_VALIDATORS_PER_COMMITTEE },
                                { spec::SYNC_COMMITTEE_SIZE },
                                { spec::MAX_PROPOSER_SLASHINGS },
                                { spec::MAX_ATTESTER_SLASHINGS },
                                { spec::MAX_ATTESTATIONS },
                                { spec::MAX_DEPOSITS },
                                { spec::MAX_VOLUNTARY_EXITS },
                            >(
                                eth1.eth1_block_hash,
                                eth1.eth1_timestamp,
                                &mut deposits,
                                context,
                            )
                            .unwrap();
                            if expected == state {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| {
                            load_initialization_test::<
                                spec::BeaconState,
                                spec::Deposit,
                                spec::ExecutionPayloadHeader, // dummy type
                            >(path)
                        },
                        |(eth1, mut deposits, execution_payload_header, expected): (
                            Eth1,
                            Vec<spec::Deposit>,
                            Option<spec::ExecutionPayloadHeader>,
                            spec::BeaconState
                        ),
                         context| {
                            let state = spec::initialize_beacon_state_from_eth1::<
                                { spec::SLOTS_PER_HISTORICAL_ROOT },
                                { spec::HISTORICAL_ROOTS_LIMIT },
                                { spec::ETH1_DATA_VOTES_BOUND },
                                { spec::VALIDATOR_REGISTRY_LIMIT },
                                { spec::EPOCHS_PER_HISTORICAL_VECTOR },
                                { spec::EPOCHS_PER_SLASHINGS_VECTOR },
                                { spec::MAX_VALIDATORS_PER_COMMITTEE },
                                { spec::SYNC_COMMITTEE_SIZE },
                                { spec::MAX_PROPOSER_SLASHINGS },
                                { spec::MAX_ATTESTER_SLASHINGS },
                                { spec::MAX_ATTESTATIONS },
                                { spec::MAX_DEPOSITS },
                                { spec::MAX_VOLUNTARY_EXITS },
                                { spec::BYTES_PER_LOGS_BLOOM },
                                { spec::MAX_EXTRA_DATA_BYTES },
                                { spec::MAX_BYTES_PER_TRANSACTION },
                                { spec::MAX_TRANSACTIONS_PER_PAYLOAD },
                            >(
                                eth1.eth1_block_hash,
                                eth1.eth1_timestamp,
                                &mut deposits,
                                execution_payload_header.as_ref(),
                                context,
                            )
                            .unwrap();
                            if expected == state {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                _ => todo!(),
            },
            _ => unreachable!(),
        },
        "validity" => match meta.config {
            Config::Minimal => match meta.fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_validity_test::<spec::BeaconState,>(path) },
                        |(state, expected): (spec::BeaconState, bool), context| {
                            let is_valid = spec::is_valid_genesis_state(&state, context);
                            if expected == is_valid {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_validity_test::<spec::BeaconState,>(path) },
                        |(state, expected): (spec::BeaconState, bool), context| {
                            let is_valid = spec::is_valid_genesis_state(&state, context);
                            if expected == is_valid {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        path,
                        meta.config,
                        |path| { load_validity_test::<spec::BeaconState,>(path) },
                        |(state, expected): (spec::BeaconState, bool), context| {
                            let is_valid = spec::is_valid_genesis_state(&state, context);
                            if expected == is_valid {
                                Ok(())
                            } else {
                                Err(Error::InvalidState)
                            }
                        }
                    }
                }
                _ => todo!(),
            },
            _ => unreachable!(),
        },
        handler => Err(Error::UnknownHandler(handler.into(), meta.name())),
    }
}
