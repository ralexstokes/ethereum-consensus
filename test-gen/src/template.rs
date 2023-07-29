use crate::{Auxillary, Spec};
use std::collections::HashMap;

pub type TestIndex = HashMap<
    String,
    HashMap<String, HashMap<String, HashMap<String, HashMap<String, HashMap<String, String>>>>>,
>;

pub fn build_index() -> HashMap<&'static str, HashMap<&'static str, Auxillary>> {
    HashMap::from([
        ("operations",
        HashMap::from([
            (
                "attestation",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::Attestation".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, operation, context| {
                    spec::process_attestation(state, operation, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "attester_slashing",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::AttesterSlashing".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, operation, context| {
                    spec::process_attester_slashing(state, operation, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "block_header",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::BeaconBlock".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, operation, context| {
                    spec::process_block_header(state, operation, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "deposit",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::Deposit".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, operation, context| {
                    spec::process_deposit(state, operation, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "proposer_slashing",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::ProposerSlashing".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, operation, context| {
                    spec::process_proposer_slashing(state, operation, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "voluntary_exit",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::SignedVoluntaryExit".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, operation, context| {
                    spec::process_voluntary_exit(state, operation, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "sync_aggregate",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::SyncAggregate".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, operation, context| {
                    spec::process_sync_aggregate(state, operation, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "execution_payload",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::ExecutionPayload".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, operation, context, execution_valid| {
                    use ethereum_consensus::state_transition::{Error, InvalidBlock, InvalidOperation, InvalidExecutionPayload};
                    let execution_engine = spec::MockExecutionEngine::new(|_| if execution_valid {
                        Ok(())
                    } else {
                        // NOTE: exact error is not specified for this test
                        Err(Error::InvalidBlock(Box::new(InvalidBlock::InvalidOperation(InvalidOperation::ExecutionPayload(InvalidExecutionPayload::InvalidTimestamp { provided: 0, expected: 0 })))))
                    });
                    spec::process_execution_payload(state, operation, execution_engine, context)
                })"
                    .to_string())]),
                },
            ),
        ])),
        ("genesis",
        HashMap::from([
            (
                "validity",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                    spec::is_valid_genesis_state(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "initialization",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::Deposit".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([
                        (Spec::Phase0, "execute(|eth1_block_hash, eth1_timestamp, deposits, _execution_payload_header, context| {
                    spec::initialize_beacon_state_from_eth1::<
                        {spec::SLOTS_PER_HISTORICAL_ROOT},
                        {spec::HISTORICAL_ROOTS_LIMIT},
                        {spec::ETH1_DATA_VOTES_BOUND},
                        {spec::VALIDATOR_REGISTRY_LIMIT},
                        {spec::EPOCHS_PER_HISTORICAL_VECTOR},
                        {spec::EPOCHS_PER_SLASHINGS_VECTOR},
                        {spec::MAX_VALIDATORS_PER_COMMITTEE},
                        {spec::PENDING_ATTESTATIONS_BOUND},
                        {spec::MAX_PROPOSER_SLASHINGS},
                        {spec::MAX_ATTESTER_SLASHINGS},
                        {spec::MAX_ATTESTATIONS},
                        {spec::MAX_DEPOSITS},
                        {spec::MAX_VOLUNTARY_EXITS},
                    >(eth1_block_hash, eth1_timestamp, deposits, context)
                })"
                    .to_string()),
                        (Spec::Altair, "execute(|eth1_block_hash, eth1_timestamp, deposits, _execution_payload_header, context| {
                    spec::initialize_beacon_state_from_eth1::<
                        {spec::SLOTS_PER_HISTORICAL_ROOT},
                        {spec::HISTORICAL_ROOTS_LIMIT},
                        {spec::ETH1_DATA_VOTES_BOUND},
                        {spec::VALIDATOR_REGISTRY_LIMIT},
                        {spec::EPOCHS_PER_HISTORICAL_VECTOR},
                        {spec::EPOCHS_PER_SLASHINGS_VECTOR},
                        {spec::MAX_VALIDATORS_PER_COMMITTEE},
                        {spec::SYNC_COMMITTEE_SIZE},
                        {spec::MAX_PROPOSER_SLASHINGS},
                        {spec::MAX_ATTESTER_SLASHINGS},
                        {spec::MAX_ATTESTATIONS},
                        {spec::MAX_DEPOSITS},
                        {spec::MAX_VOLUNTARY_EXITS},
                        >(eth1_block_hash, eth1_timestamp, deposits, context)
                })"
                    .to_string()),
                        (Spec::Bellatrix, "execute(|eth1_block_hash, eth1_timestamp, deposits, execution_payload_header, context| {
                    spec::initialize_beacon_state_from_eth1::<
                        {spec::SLOTS_PER_HISTORICAL_ROOT},
                        {spec::HISTORICAL_ROOTS_LIMIT},
                        {spec::ETH1_DATA_VOTES_BOUND},
                        {spec::VALIDATOR_REGISTRY_LIMIT},
                        {spec::EPOCHS_PER_HISTORICAL_VECTOR},
                        {spec::EPOCHS_PER_SLASHINGS_VECTOR},
                        {spec::MAX_VALIDATORS_PER_COMMITTEE},
                        {spec::SYNC_COMMITTEE_SIZE},
                        {spec::MAX_PROPOSER_SLASHINGS},
                        {spec::MAX_ATTESTER_SLASHINGS},
                        {spec::MAX_ATTESTATIONS},
                        {spec::MAX_DEPOSITS},
                        {spec::MAX_VOLUNTARY_EXITS},
                        {spec::BYTES_PER_LOGS_BLOOM},
                        {spec::MAX_EXTRA_DATA_BYTES},
                        {spec::MAX_BYTES_PER_TRANSACTION},
                        {spec::MAX_TRANSACTIONS_PER_PAYLOAD},
                        >(eth1_block_hash, eth1_timestamp, deposits, execution_payload_header, context)
                })"
                    .to_string())
                    ]),
                },
            ),
        ])),
        ("transition",
        HashMap::from([
            (
                "core",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([
                        (Spec::Altair, "execute(|state: pre_spec::BeaconState, pre_blocks: Vec<pre_spec::SignedBeaconBlock>, blocks: Vec<spec::SignedBeaconBlock>, context| {
                    let mut executor = Executor::new(state.into(), NoOpExecutionEngine, context);
                    for block in pre_blocks.into_iter() {
                        let mut block = block.into();
                        executor.apply_block(&mut block)?;
                    }
                    for block in blocks.into_iter() {
                        let mut block = block.into();
                        executor.apply_block(&mut block)?;
                    }
                    match executor.state {
                        BeaconState::Altair(inner) => Ok(*inner),
                        _ => unreachable!(),
                    }
                })"
                    .to_string()),
                        (Spec::Bellatrix, "execute(|state: pre_spec::BeaconState, pre_blocks: Vec<pre_spec::SignedBeaconBlock>, blocks: Vec<spec::SignedBeaconBlock>, context| {
                    let mut executor = Executor::new(state.into(), NoOpExecutionEngine, context);
                    for block in pre_blocks.into_iter() {
                        let mut block = block.into();
                        executor.apply_block(&mut block)?;
                    }
                    for block in blocks.into_iter() {
                        let mut block = block.into();
                        executor.apply_block(&mut block)?;
                    }
                    match executor.state {
                        BeaconState::Bellatrix(inner) => Ok(*inner),
                        _ => unreachable!(),
                    }
                })"
                    .to_string())]),
                },
            ),
        ])),
        ("fork",
        HashMap::from([
            (
                "fork",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([
                        (Spec::Altair, "execute(|state: &phase0::BeaconState, context| -> spec::BeaconState {
                    spec::upgrade_to_altair(state, context).unwrap()
                })"
                    .to_string()),
                        (Spec::Bellatrix, "execute(|state: &altair::BeaconState, context| -> spec::BeaconState {
                    spec::upgrade_to_bellatrix(state, context)
                })"
                    .to_string())
                    ]),
                },
            ),
        ])),
        ("sanity",
        HashMap::from([
            (
                "slots",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, offset, context| {
                    let target_slot = state.slot + offset;
                    spec::process_slots(state, target_slot, context).unwrap();
                })"
                    .to_string())]),
                },
            ),
            (
                "blocks",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::SignedBeaconBlock"
                        .to_string(),
                    preamble: HashMap::from_iter([(Spec::Bellatrix, "let execution_engine = spec::NoOpExecutionEngine;".to_string())]),
                    execution_handler: HashMap::from_iter([(Spec::Phase0, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Altair, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Bellatrix, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, execution_engine.clone(), validation, context)?;
                        }
                        Ok(())
                    })".to_string())]),
                },
            ),
        ])),
        ("rewards",
        HashMap::from([
            (
                "basic",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([
                        (Spec::Phase0, "execute(|state, context| {
                    let source_deltas = spec::get_source_deltas(state, context).unwrap();
                    let target_deltas = spec::get_target_deltas(state, context).unwrap();
                    let head_deltas = spec::get_head_deltas(state, context).unwrap();
                    let inclusion_penalty_deltas = spec::get_inclusion_delay_deltas(state, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, Some(inclusion_penalty_deltas), inactivity_penalty_deltas)
                })"
                    .to_string()),
                        (Spec::Altair, "execute(|state, context| {
                    let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                    let source_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                    let target_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                    let head_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                })"
                    .to_string()),
                        (Spec::Bellatrix, "execute(|state, context| {
                    let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                    let source_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                    let target_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                    let head_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                })"
                    .to_string()),
                    ]),
                },
            ),
            (
                "leak",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([
                        (Spec::Phase0, "execute(|state, context| {
                    let source_deltas = spec::get_source_deltas(state, context).unwrap();
                    let target_deltas = spec::get_target_deltas(state, context).unwrap();
                    let head_deltas = spec::get_head_deltas(state, context).unwrap();
                    let inclusion_penalty_deltas = spec::get_inclusion_delay_deltas(state, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, Some(inclusion_penalty_deltas), inactivity_penalty_deltas)
                })"
                    .to_string()),
                        (Spec::Altair, "execute(|state, context| {
                    let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                    let source_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                    let target_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                    let head_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                })"
                    .to_string()),
                        (Spec::Bellatrix, "execute(|state, context| {
                    let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                    let source_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                    let target_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                    let head_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                })"
                    .to_string()),
                    ]),
                },
            ),
            (
                "random",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([
                        (Spec::Phase0, "execute(|state, context| {
                    let source_deltas = spec::get_source_deltas(state, context).unwrap();
                    let target_deltas = spec::get_target_deltas(state, context).unwrap();
                    let head_deltas = spec::get_head_deltas(state, context).unwrap();
                    let inclusion_penalty_deltas = spec::get_inclusion_delay_deltas(state, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, Some(inclusion_penalty_deltas), inactivity_penalty_deltas)
                })"
                    .to_string()),
                        (Spec::Altair, "execute(|state, context| {
                    let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                    let source_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                    let target_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                    let head_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                })"
                    .to_string()),
                        (Spec::Bellatrix, "execute(|state, context| {
                    let flag_index = spec::TIMELY_SOURCE_FLAG_INDEX;
                    let source_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_TARGET_FLAG_INDEX;
                    let target_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let flag_index = spec::TIMELY_HEAD_FLAG_INDEX;
                    let head_deltas = spec::get_flag_index_deltas(state, flag_index, context).unwrap();
                    let inactivity_penalty_deltas = spec::get_inactivity_penalty_deltas(state, context).unwrap();
                    (source_deltas, target_deltas, head_deltas, None, inactivity_penalty_deltas)
                })"
                    .to_string()),
                    ]),
                },
            ),
        ])),
        ("finality",
        HashMap::from([
            (
                "finality",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::SignedBeaconBlock"
                        .to_string(),
                    preamble: HashMap::from_iter([(Spec::Bellatrix, "let execution_engine = spec::NoOpExecutionEngine;".to_string())]),
                    execution_handler: HashMap::from_iter([(Spec::Phase0, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Altair, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Bellatrix, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, execution_engine.clone(), validation, context)?;
                        }
                        Ok(())
                    })".to_string())]),
                },
            ),
        ])),
        ("ssz_static",
        HashMap::from([
            (
                "aggregate_and_proof",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::AggregateAndProof = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "attestation",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::Attestation = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "attestation_data",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::AttestationData = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "attester_slashing",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::AttesterSlashing = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "beacon_block",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::BeaconBlock = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "beacon_block_body",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::BeaconBlockBody = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "beacon_block_header",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::BeaconBlockHeader = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "beacon_state",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::BeaconState = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "checkpoint",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::Checkpoint = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "contribution_and_proof",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::ContributionAndProof = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "deposit",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::Deposit = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "deposit_data",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::DepositData = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "deposit_message",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::DepositMessage = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "eth_1_block",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::Eth1Block = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "eth_1_data",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::Eth1Data = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "execution_payload",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::ExecutionPayload = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "execution_payload_header",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::ExecutionPayloadHeader = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "fork",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::Fork = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "fork_data",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::ForkData = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "historical_batch",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::HistoricalBatch = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "indexed_attestation",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::IndexedAttestation = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "light_client_update",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::LightClientUpdate = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "pending_attestation",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::PendingAttestation = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "pow_block",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::PowBlock = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "proposer_slashing",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::ProposerSlashing = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "signed_aggregate_and_proof",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SignedAggregateAndProof = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "signed_beacon_block",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SignedBeaconBlock = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "signed_beacon_block_header",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SignedBeaconBlockHeader = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "signed_contribution_and_proof",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SignedContributionAndProof = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "signed_voluntary_exit",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SignedVoluntaryExit = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "signing_data",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SigningData = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "sync_aggregate",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SyncAggregate = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "sync_aggregator_selection_data",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SyncAggregatorSelectionData = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "sync_committee",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SyncCommittee = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "sync_committee_contribution",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SyncCommitteeContribution = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "sync_committee_message",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::SyncCommitteeMessage = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "validator",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::Validator = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
            (
                "voluntary_exit",
                Auxillary {
                    test_case_type_generics: Default::default(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|encoding| {
                        let mut data: spec::VoluntaryExit = ssz_rs::deserialize(encoding).unwrap();
                        let serialized = ssz_rs::serialize(&data).unwrap();
                        let root = data.hash_tree_root().unwrap();
                        (serialized, root)
                })"
                    .to_string())]),
                },
            ),
        ])),
        ("random",
        HashMap::from([
            (
                "random",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::SignedBeaconBlock"
                        .to_string(),
                    preamble: HashMap::from_iter([(Spec::Bellatrix, "let execution_engine = spec::NoOpExecutionEngine;".to_string())]),
                    execution_handler: HashMap::from_iter([(Spec::Phase0, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Altair, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Bellatrix, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, execution_engine.clone(), validation, context)?;
                        }
                        Ok(())
                    })".to_string())]),
                },
            ),
        ])),
        ("epoch_processing",
        HashMap::from([
            (
                "effective_balance_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_effective_balance_updates(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "eth_1_data_reset",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_eth1_data_reset(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "historical_roots_update",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_historical_roots_update(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "inactivity_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_inactivity_updates(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "justification_and_finalization",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_justification_and_finalization(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "participation_record_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_participation_record_updates(state);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "participation_flag_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_participation_flag_updates(state)
                })"
                    .to_string())]),
                },
            ),
            (
                "randao_mixes_reset",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_randao_mixes_reset(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "registry_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_registry_updates(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "rewards_and_penalties",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_rewards_and_penalties(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "slashings",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_slashings(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "slashings_reset",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_slashings_reset(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "sync_committee_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_sync_committee_updates(state, context)
                })"
                    .to_string())]),
                },
            ),
        ])),
    ])
}
