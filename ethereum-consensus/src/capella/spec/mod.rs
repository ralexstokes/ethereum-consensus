//! WARNING: This file was derived by the `spec-gen` utility. DO NOT EDIT MANUALLY.
pub use crate::{
    altair::{
        constants::{
            PARTICIPATION_FLAG_WEIGHTS, PROPOSER_WEIGHT, SYNC_COMMITTEE_SUBNET_COUNT,
            SYNC_REWARD_WEIGHT, TIMELY_HEAD_FLAG_INDEX, TIMELY_HEAD_WEIGHT,
            TIMELY_SOURCE_FLAG_INDEX, TIMELY_SOURCE_WEIGHT, TIMELY_TARGET_FLAG_INDEX,
            TIMELY_TARGET_WEIGHT, WEIGHT_DENOMINATOR,
        },
        light_client::{
            LightClientBootstrap, LightClientFinalityUpdate, LightClientHeader,
            LightClientOptimisticUpdate, LightClientStore, LightClientUpdate,
            CURRENT_SYNC_COMMITTEE_INDEX, CURRENT_SYNC_COMMITTEE_INDEX_FLOOR_LOG_2,
            FINALIZED_ROOT_INDEX, FINALIZED_ROOT_INDEX_FLOOR_LOG_2, NEXT_SYNC_COMMITTEE_INDEX,
            NEXT_SYNC_COMMITTEE_INDEX_FLOOR_LOG_2,
        },
        sync::{SyncAggregate, SyncCommittee},
        validator::{
            ContributionAndProof, SignedContributionAndProof, SyncAggregatorSelectionData,
            SyncCommitteeContribution, SyncCommitteeMessage,
        },
    },
    bellatrix::{execution::Transaction, fork_choice::PowBlock},
    capella::{
        beacon_block::{BeaconBlock, BeaconBlockBody, SignedBeaconBlock},
        beacon_state::BeaconState,
        blinded_beacon_block::{
            BlindedBeaconBlock, BlindedBeaconBlockBody, SignedBlindedBeaconBlock,
        },
        bls_to_execution_change::{BlsToExecutionChange, SignedBlsToExecutionChange},
        epoch_processing::{process_epoch, process_historical_summaries_update},
        execution::{
            ExecutionPayload, ExecutionPayloadHeader, MockExecutionEngine, NoOpExecutionEngine,
        },
        helpers::{
            has_eth1_withdrawal_credential, is_fully_withdrawable_validator,
            is_partially_withdrawable_validator,
        },
        withdrawal::Withdrawal,
    },
    phase0::{
        beacon_block::{BeaconBlockHeader, SignedBeaconBlockHeader},
        beacon_state::{HistoricalBatch, HistoricalSummary},
        constants::{
            BASE_REWARDS_PER_EPOCH, DEPOSIT_CONTRACT_TREE_DEPTH, DEPOSIT_DATA_LIST_BOUND,
            JUSTIFICATION_BITS_LENGTH,
        },
        fork::{Fork, ForkData},
        operations::{
            Attestation, AttestationData, AttesterSlashing, Checkpoint, Deposit, DepositData,
            DepositMessage, Eth1Data, IndexedAttestation, PendingAttestation, ProposerSlashing,
            SignedVoluntaryExit, VoluntaryExit,
        },
        validator::{AggregateAndProof, Eth1Block, SignedAggregateAndProof, Validator},
    },
    primitives::*,
    signing::*,
    state_transition::*,
};
use crate::{
    crypto::{
        eth_aggregate_public_keys, eth_fast_aggregate_verify, fast_aggregate_verify, hash,
        verify_signature,
    },
    ssz::*,
};
use integer_sqrt::IntegerSquareRoot;
use ssz_rs::prelude::*;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    iter::zip,
    mem,
};
pub fn process_execution_payload<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    payload: &mut ExecutionPayload<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >,
    execution_engine: E,
    context: &Context,
) -> Result<()> {
    let parent_hash_invalid =
        payload.parent_hash != state.latest_execution_payload_header.block_hash;
    if is_merge_transition_complete(state) && parent_hash_invalid {
        return Err(invalid_operation_error(
            InvalidExecutionPayload::InvalidParentHash {
                provided: payload.parent_hash.clone(),
                expected: state.latest_execution_payload_header.block_hash.clone(),
            }
            .into(),
        ))
    }
    let current_epoch = get_current_epoch(state, context);
    let randao_mix = get_randao_mix(state, current_epoch);
    if &payload.prev_randao != randao_mix {
        return Err(invalid_operation_error(
            InvalidExecutionPayload::InvalidPrevRandao {
                provided: payload.prev_randao.clone(),
                expected: randao_mix.clone(),
            }
            .into(),
        ))
    }
    let timestamp = compute_timestamp_at_slot(state, state.slot, context)?;
    if payload.timestamp != timestamp {
        return Err(invalid_operation_error(
            InvalidExecutionPayload::InvalidTimestamp {
                provided: payload.timestamp,
                expected: timestamp,
            }
            .into(),
        ))
    }
    execution_engine.notify_new_payload(payload)?;
    state.latest_execution_payload_header = ExecutionPayloadHeader {
        parent_hash: payload.parent_hash.clone(),
        fee_recipient: payload.fee_recipient.clone(),
        state_root: payload.state_root.clone(),
        receipts_root: payload.receipts_root.clone(),
        logs_bloom: payload.logs_bloom.clone(),
        prev_randao: payload.prev_randao.clone(),
        block_number: payload.block_number,
        gas_limit: payload.gas_limit,
        gas_used: payload.gas_used,
        timestamp: payload.timestamp,
        extra_data: payload.extra_data.clone(),
        base_fee_per_gas: payload.base_fee_per_gas.clone(),
        block_hash: payload.block_hash.clone(),
        transactions_root: payload.transactions.hash_tree_root()?,
    };
    Ok(())
}
pub fn process_block<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    block: &mut BeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
    execution_engine: E,
    context: &Context,
) -> Result<()> {
    process_block_header(state, block, context)?;
    if is_execution_enabled(state, &block.body) {
        process_execution_payload(
            state,
            &mut block.body.execution_payload,
            execution_engine,
            context,
        )?;
    }
    process_randao(state, &block.body, context)?;
    process_eth1_data(state, &block.body, context);
    process_operations(state, &mut block.body, context)?;
    process_sync_aggregate(state, &block.body.sync_aggregate, context)?;
    Ok(())
}
pub fn process_attestation<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    attestation: &Attestation<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<()> {
    let data = &attestation.data;
    let is_previous = data.target.epoch == get_previous_epoch(state, context);
    let current_epoch = get_current_epoch(state, context);
    let is_current = data.target.epoch == current_epoch;
    let valid_target_epoch = is_previous || is_current;
    if !valid_target_epoch {
        return Err(invalid_operation_error(InvalidOperation::Attestation(
            InvalidAttestation::InvalidTargetEpoch {
                target: data.target.epoch,
                current: current_epoch,
            },
        )))
    }
    let attestation_epoch = compute_epoch_at_slot(data.slot, context);
    if data.target.epoch != attestation_epoch {
        return Err(invalid_operation_error(InvalidOperation::Attestation(
            InvalidAttestation::InvalidSlot {
                slot: data.slot,
                epoch: attestation_epoch,
                target: data.target.epoch,
            },
        )))
    }
    let attestation_has_delay = data.slot + context.min_attestation_inclusion_delay <= state.slot;
    let attestation_is_recent = state.slot <= data.slot + context.slots_per_epoch;
    let attestation_is_timely = attestation_has_delay && attestation_is_recent;
    if !attestation_is_timely {
        return Err(invalid_operation_error(InvalidOperation::Attestation(
            InvalidAttestation::NotTimely {
                state_slot: state.slot,
                attestation_slot: data.slot,
                lower_bound: data.slot + context.slots_per_epoch,
                upper_bound: data.slot + context.min_attestation_inclusion_delay,
            },
        )))
    }
    let committee_count = get_committee_count_per_slot(state, data.target.epoch, context);
    if data.index >= committee_count {
        return Err(invalid_operation_error(InvalidOperation::Attestation(
            InvalidAttestation::InvalidIndex { index: data.index, upper_bound: committee_count },
        )))
    }
    let committee = get_beacon_committee(state, data.slot, data.index, context)?;
    if attestation.aggregation_bits.len() != committee.len() {
        return Err(invalid_operation_error(InvalidOperation::Attestation(
            InvalidAttestation::Bitfield {
                expected_length: committee.len(),
                length: attestation.aggregation_bits.len(),
            },
        )))
    }
    let inclusion_delay = state.slot - data.slot;
    let participation_flag_indices =
        get_attestation_participation_flag_indices(state, data, inclusion_delay, context)?;
    is_valid_indexed_attestation(
        state,
        &mut get_indexed_attestation(state, attestation, context)?,
        context,
    )?;
    let attesting_indices =
        get_attesting_indices(state, data, &attestation.aggregation_bits, context)?;
    let mut proposer_reward_numerator = 0;
    for index in attesting_indices {
        for (flag_index, weight) in PARTICIPATION_FLAG_WEIGHTS.iter().enumerate() {
            if is_current {
                if participation_flag_indices.contains(&flag_index) &&
                    !has_flag(state.current_epoch_participation[index], flag_index)
                {
                    state.current_epoch_participation[index] =
                        add_flag(state.current_epoch_participation[index], flag_index);
                    proposer_reward_numerator += get_base_reward(state, index, context)? * weight;
                }
            } else if participation_flag_indices.contains(&flag_index) &&
                !has_flag(state.previous_epoch_participation[index], flag_index)
            {
                state.previous_epoch_participation[index] =
                    add_flag(state.previous_epoch_participation[index], flag_index);
                proposer_reward_numerator += get_base_reward(state, index, context)? * weight;
            }
        }
    }
    let proposer_reward_denominator =
        (WEIGHT_DENOMINATOR - PROPOSER_WEIGHT) * WEIGHT_DENOMINATOR / PROPOSER_WEIGHT;
    let proposer_reward = proposer_reward_numerator / proposer_reward_denominator;
    increase_balance(state, get_beacon_proposer_index(state, context)?, proposer_reward);
    Ok(())
}
pub fn process_deposit<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    deposit: &mut Deposit,
    context: &Context,
) -> Result<()> {
    let branch = deposit
        .proof
        .iter()
        .map(|node| Node::try_from(node.as_ref()).expect("is valid instance"))
        .collect::<Vec<_>>();
    let leaf = deposit.data.hash_tree_root()?;
    let depth = DEPOSIT_CONTRACT_TREE_DEPTH + 1;
    let index = state.eth1_deposit_index as usize;
    let root = &state.eth1_data.deposit_root;
    if !is_valid_merkle_branch(&leaf, branch.iter(), depth, index, root) {
        return Err(invalid_operation_error(InvalidOperation::Deposit(
            InvalidDeposit::InvalidProof { leaf, branch, depth, index, root: *root },
        )))
    }
    state.eth1_deposit_index += 1;
    let public_key = &deposit.data.public_key;
    let amount = deposit.data.amount;
    let validator_public_keys: HashSet<&BlsPublicKey> =
        HashSet::from_iter(state.validators.iter().map(|v| &v.public_key));
    if !validator_public_keys.contains(public_key) {
        let mut deposit_message = DepositMessage {
            public_key: public_key.clone(),
            withdrawal_credentials: deposit.data.withdrawal_credentials.clone(),
            amount,
        };
        let domain = compute_domain(DomainType::Deposit, None, None, context)?;
        let signing_root = compute_signing_root(&mut deposit_message, domain)?;
        if verify_signature(public_key, signing_root.as_ref(), &deposit.data.signature).is_err() {
            return Ok(())
        }
        state.validators.push(get_validator_from_deposit(deposit, context));
        state.balances.push(amount);
        state.previous_epoch_participation.push(ParticipationFlags::default());
        state.current_epoch_participation.push(ParticipationFlags::default());
        state.inactivity_scores.push(0)
    } else {
        let index = state.validators.iter().position(|v| &v.public_key == public_key).unwrap();
        increase_balance(state, index, amount);
    }
    Ok(())
}
pub fn process_sync_aggregate<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    sync_aggregate: &SyncAggregate<SYNC_COMMITTEE_SIZE>,
    context: &Context,
) -> Result<()> {
    let committee_public_keys = &state.current_sync_committee.public_keys;
    let participant_public_keys =
        zip(committee_public_keys.iter(), sync_aggregate.sync_committee_bits.iter())
            .filter_map(|(public_key, bit)| if *bit { Some(public_key) } else { None })
            .collect::<Vec<_>>();
    let previous_slot = u64::max(state.slot, 1) - 1;
    let domain = get_domain(
        state,
        DomainType::SyncCommittee,
        Some(compute_epoch_at_slot(previous_slot, context)),
        context,
    )?;
    let mut root_at_slot = *get_block_root_at_slot(state, previous_slot)?;
    let signing_root = compute_signing_root(&mut root_at_slot, domain)?;
    if eth_fast_aggregate_verify(
        participant_public_keys.as_slice(),
        signing_root.as_ref(),
        &sync_aggregate.sync_committee_signature,
    )
    .is_err()
    {
        return Err(invalid_operation_error(InvalidOperation::SyncAggregate(
            InvalidSyncAggregate::InvalidSignature {
                signature: sync_aggregate.sync_committee_signature.clone(),
                root: signing_root,
            },
        )))
    }
    let total_active_increments =
        get_total_active_balance(state, context)? / context.effective_balance_increment;
    let total_base_rewards =
        get_base_reward_per_increment(state, context)? * total_active_increments;
    let max_participant_rewards =
        total_base_rewards * SYNC_REWARD_WEIGHT / WEIGHT_DENOMINATOR / context.slots_per_epoch;
    let participant_reward = max_participant_rewards / context.sync_committee_size as u64;
    let proposer_reward =
        participant_reward * PROPOSER_WEIGHT / (WEIGHT_DENOMINATOR - PROPOSER_WEIGHT);
    let all_public_keys = state
        .validators
        .iter()
        .enumerate()
        .map(|(i, v)| (&v.public_key, i))
        .collect::<HashMap<&BlsPublicKey, usize>>();
    let mut committee_indices: Vec<ValidatorIndex> = Vec::default();
    for public_key in state.current_sync_committee.public_keys.iter() {
        committee_indices
            .push(*all_public_keys.get(public_key).expect("validator public_key should exist"));
    }
    for (participant_index, participation_bit) in
        zip(committee_indices.iter(), sync_aggregate.sync_committee_bits.iter())
    {
        if *participation_bit {
            increase_balance(state, *participant_index, participant_reward);
            increase_balance(state, get_beacon_proposer_index(state, context)?, proposer_reward);
        } else {
            decrease_balance(state, *participant_index, participant_reward);
        }
    }
    Ok(())
}
pub fn process_proposer_slashing<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    proposer_slashing: &mut ProposerSlashing,
    context: &Context,
) -> Result<()> {
    let header_1 = &proposer_slashing.signed_header_1.message;
    let header_2 = &proposer_slashing.signed_header_2.message;
    if header_1.slot != header_2.slot {
        return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::SlotMismatch(header_1.slot, header_2.slot),
        )))
    }
    if header_1.proposer_index != header_2.proposer_index {
        return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::ProposerMismatch(
                header_1.proposer_index,
                header_2.proposer_index,
            ),
        )))
    }
    if header_1 == header_2 {
        return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::HeadersAreEqual(header_1.clone()),
        )))
    }
    let proposer_index = header_1.proposer_index;
    let proposer = state.validators.get(proposer_index).ok_or_else(|| {
        invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::InvalidIndex(proposer_index),
        ))
    })?;
    if !is_slashable_validator(proposer, get_current_epoch(state, context)) {
        return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
            InvalidProposerSlashing::ProposerIsNotSlashable(header_1.proposer_index),
        )))
    }
    let epoch = compute_epoch_at_slot(header_1.slot, context);
    let domain = get_domain(state, DomainType::BeaconProposer, Some(epoch), context)?;
    for signed_header in
        [&mut proposer_slashing.signed_header_1, &mut proposer_slashing.signed_header_2]
    {
        let signing_root = compute_signing_root(&mut signed_header.message, domain)?;
        let public_key = &proposer.public_key;
        if verify_signature(public_key, signing_root.as_ref(), &signed_header.signature).is_err() {
            return Err(invalid_operation_error(InvalidOperation::ProposerSlashing(
                InvalidProposerSlashing::InvalidSignature(signed_header.signature.clone()),
            )))
        }
    }
    slash_validator(state, proposer_index, None, context)
}
pub fn process_attester_slashing<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    attester_slashing: &mut AttesterSlashing<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<()> {
    let attestation_1 = &mut attester_slashing.attestation_1;
    let attestation_2 = &mut attester_slashing.attestation_2;
    if !is_slashable_attestation_data(&attestation_1.data, &attestation_2.data) {
        return Err(invalid_operation_error(InvalidOperation::AttesterSlashing(
            InvalidAttesterSlashing::NotSlashable(
                Box::new(attestation_1.data.clone()),
                Box::new(attestation_2.data.clone()),
            ),
        )))
    }
    is_valid_indexed_attestation(state, attestation_1, context)?;
    is_valid_indexed_attestation(state, attestation_2, context)?;
    let indices_1: HashSet<ValidatorIndex> =
        HashSet::from_iter(attestation_1.attesting_indices.iter().cloned());
    let indices_2 = HashSet::from_iter(attestation_2.attesting_indices.iter().cloned());
    let mut indices = indices_1.intersection(&indices_2).cloned().collect::<Vec<_>>();
    indices.sort_unstable();
    let mut slashed_any = false;
    let current_epoch = get_current_epoch(state, context);
    for &index in &indices {
        if is_slashable_validator(&state.validators[index], current_epoch) {
            slash_validator(state, index, None, context)?;
            slashed_any = true;
        }
    }
    if !slashed_any {
        Err(invalid_operation_error(InvalidOperation::AttesterSlashing(
            InvalidAttesterSlashing::NoSlashings(indices),
        )))
    } else {
        Ok(())
    }
}
pub fn get_validator_from_deposit(deposit: &Deposit, context: &Context) -> Validator {
    let amount = deposit.data.amount;
    let effective_balance = Gwei::min(
        amount - amount % context.effective_balance_increment,
        context.max_effective_balance,
    );
    Validator {
        public_key: deposit.data.public_key.clone(),
        withdrawal_credentials: deposit.data.withdrawal_credentials.clone(),
        effective_balance,
        activation_eligibility_epoch: FAR_FUTURE_EPOCH,
        activation_epoch: FAR_FUTURE_EPOCH,
        exit_epoch: FAR_FUTURE_EPOCH,
        withdrawable_epoch: FAR_FUTURE_EPOCH,
        ..Default::default()
    }
}
pub fn process_voluntary_exit<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    signed_voluntary_exit: &mut SignedVoluntaryExit,
    context: &Context,
) -> Result<()> {
    let voluntary_exit = &mut signed_voluntary_exit.message;
    let validator = state.validators.get(voluntary_exit.validator_index).ok_or_else(|| {
        invalid_operation_error(InvalidOperation::VoluntaryExit(
            InvalidVoluntaryExit::InvalidIndex(voluntary_exit.validator_index),
        ))
    })?;
    let current_epoch = get_current_epoch(state, context);
    if !is_active_validator(validator, current_epoch) {
        return Err(invalid_operation_error(InvalidOperation::VoluntaryExit(
            InvalidVoluntaryExit::InactiveValidator(current_epoch),
        )))
    }
    if validator.exit_epoch != FAR_FUTURE_EPOCH {
        return Err(invalid_operation_error(InvalidOperation::VoluntaryExit(
            InvalidVoluntaryExit::ValidatorAlreadyExited {
                index: voluntary_exit.validator_index,
                epoch: validator.exit_epoch,
            },
        )))
    }
    if current_epoch < voluntary_exit.epoch {
        return Err(invalid_operation_error(InvalidOperation::VoluntaryExit(
            InvalidVoluntaryExit::EarlyExit { current_epoch, exit_epoch: voluntary_exit.epoch },
        )))
    }
    let minimum_time_active =
        validator.activation_eligibility_epoch + context.shard_committee_period;
    if current_epoch < minimum_time_active {
        return Err(invalid_operation_error(InvalidOperation::VoluntaryExit(
            InvalidVoluntaryExit::ValidatorIsNotActiveForLongEnough {
                current_epoch,
                minimum_time_active,
            },
        )))
    }
    let domain = get_domain(state, DomainType::VoluntaryExit, Some(voluntary_exit.epoch), context)?;
    let signing_root = compute_signing_root(voluntary_exit, domain)?;
    let public_key = &validator.public_key;
    if verify_signature(public_key, signing_root.as_ref(), &signed_voluntary_exit.signature)
        .is_err()
    {
        return Err(invalid_operation_error(InvalidOperation::VoluntaryExit(
            InvalidVoluntaryExit::InvalidSignature(signed_voluntary_exit.signature.clone()),
        )))
    }
    initiate_validator_exit(state, voluntary_exit.validator_index, context);
    Ok(())
}
pub fn process_block_header<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    block: &mut BeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
    context: &Context,
) -> Result<()> {
    if block.slot != state.slot {
        return Err(invalid_header_error(InvalidBeaconBlockHeader::StateSlotMismatch {
            state_slot: state.slot,
            block_slot: block.slot,
        }))
    }
    if block.slot <= state.latest_block_header.slot {
        return Err(invalid_header_error(InvalidBeaconBlockHeader::OlderThanLatestBlockHeader {
            block_slot: block.slot,
            latest_block_header_slot: state.latest_block_header.slot,
        }))
    }
    let proposer_index = get_beacon_proposer_index(state, context)?;
    if block.proposer_index != proposer_index {
        return Err(invalid_header_error(InvalidBeaconBlockHeader::ProposerIndexMismatch {
            block_proposer_index: block.proposer_index,
            proposer_index,
        }))
    }
    let expected_parent_root = state.latest_block_header.hash_tree_root()?;
    if block.parent_root != expected_parent_root {
        return Err(invalid_header_error(InvalidBeaconBlockHeader::ParentBlockRootMismatch {
            expected: expected_parent_root,
            provided: block.parent_root,
        }))
    }
    state.latest_block_header = BeaconBlockHeader {
        slot: block.slot,
        proposer_index: block.proposer_index,
        parent_root: block.parent_root,
        body_root: block.body.hash_tree_root()?,
        ..Default::default()
    };
    let proposer = &state.validators[block.proposer_index];
    if proposer.slashed {
        return Err(invalid_header_error(InvalidBeaconBlockHeader::ProposerSlashed(proposer_index)))
    }
    Ok(())
}
pub fn xor(a: &Bytes32, b: &Bytes32) -> Bytes32 {
    let inner = a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect::<Vec<_>>();
    ByteVector::<32>::try_from(inner.as_ref()).unwrap()
}
pub fn process_randao<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    body: &BeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
    context: &Context,
) -> Result<()> {
    let mut epoch = get_current_epoch(state, context);
    let proposer_index = get_beacon_proposer_index(state, context)?;
    let proposer = &state.validators[proposer_index];
    let domain = get_domain(state, DomainType::Randao, Some(epoch), context)?;
    let signing_root = compute_signing_root(&mut epoch, domain)?;
    if verify_signature(&proposer.public_key, signing_root.as_ref(), &body.randao_reveal).is_err() {
        return Err(invalid_operation_error(InvalidOperation::Randao(body.randao_reveal.clone())))
    }
    let mix = xor(get_randao_mix(state, epoch), &hash(body.randao_reveal.as_ref()));
    let mix_index = epoch % context.epochs_per_historical_vector;
    state.randao_mixes[mix_index as usize] = mix;
    Ok(())
}
pub fn process_eth1_data<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    body: &BeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
    context: &Context,
) {
    state.eth1_data_votes.push(body.eth1_data.clone());
    let votes_count =
        state.eth1_data_votes.iter().filter(|&vote| *vote == body.eth1_data).count() as u64;
    if votes_count * 2 > context.epochs_per_eth1_voting_period * context.slots_per_epoch {
        state.eth1_data = body.eth1_data.clone();
    }
}
pub fn process_operations<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    body: &mut BeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
    context: &Context,
) -> Result<()> {
    let expected_deposit_count = usize::min(
        context.max_deposits,
        (state.eth1_data.deposit_count - state.eth1_deposit_index) as usize,
    );
    if body.deposits.len() != expected_deposit_count {
        return Err(invalid_operation_error(InvalidOperation::Deposit(
            InvalidDeposit::IncorrectCount {
                expected: expected_deposit_count,
                count: body.deposits.len(),
            },
        )))
    }
    body.proposer_slashings
        .iter_mut()
        .try_for_each(|op| process_proposer_slashing(state, op, context))?;
    body.attester_slashings
        .iter_mut()
        .try_for_each(|op| process_attester_slashing(state, op, context))?;
    body.attestations.iter().try_for_each(|op| process_attestation(state, op, context))?;
    body.deposits.iter_mut().try_for_each(|op| process_deposit(state, op, context))?;
    body.voluntary_exits
        .iter_mut()
        .try_for_each(|op| process_voluntary_exit(state, op, context))?;
    Ok(())
}
pub fn process_slashings<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<()> {
    let epoch = get_current_epoch(state, context);
    let total_balance = get_total_active_balance(state, context)?;
    let adjusted_total_slashing_balance = Gwei::min(
        state.slashings.iter().sum::<Gwei>() * context.proportional_slashing_multiplier_bellatrix,
        total_balance,
    );
    for i in 0..state.validators.len() {
        let validator = &state.validators[i];
        if validator.slashed &&
            (epoch + context.epochs_per_slashings_vector / 2) == validator.withdrawable_epoch
        {
            let increment = context.effective_balance_increment;
            let penalty_numerator =
                validator.effective_balance / increment * adjusted_total_slashing_balance;
            let penalty = penalty_numerator / total_balance * increment;
            decrease_balance(state, i, penalty);
        }
    }
    Ok(())
}
pub fn get_base_reward<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    index: ValidatorIndex,
    context: &Context,
) -> Result<Gwei> {
    let increments =
        state.validators[index].effective_balance / context.effective_balance_increment;
    Ok(increments * get_base_reward_per_increment(state, context)?)
}
pub fn process_justification_and_finalization<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<()> {
    let current_epoch = get_current_epoch(state, context);
    if current_epoch <= GENESIS_EPOCH + 1 {
        return Ok(())
    }
    let previous_indices = get_unslashed_participating_indices(
        state,
        TIMELY_TARGET_FLAG_INDEX,
        get_previous_epoch(state, context),
        context,
    )?;
    let current_indices = get_unslashed_participating_indices(
        state,
        TIMELY_TARGET_FLAG_INDEX,
        current_epoch,
        context,
    )?;
    let total_active_balance = get_total_active_balance(state, context)?;
    let previous_target_balance = get_total_balance(state, &previous_indices, context)?;
    let current_target_balance = get_total_balance(state, &current_indices, context)?;
    weigh_justification_and_finalization(
        state,
        total_active_balance,
        previous_target_balance,
        current_target_balance,
        context,
    )
}
pub fn process_inactivity_updates<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<()> {
    let current_epoch = get_current_epoch(state, context);
    if current_epoch == GENESIS_EPOCH {
        return Ok(())
    }
    let eligible_validator_indices =
        get_eligible_validator_indices(state, context).collect::<Vec<_>>();
    let unslashed_participating_indices = get_unslashed_participating_indices(
        state,
        TIMELY_TARGET_FLAG_INDEX,
        get_previous_epoch(state, context),
        context,
    )?;
    let not_is_leaking = !is_in_inactivity_leak(state, context);
    for index in eligible_validator_indices {
        if unslashed_participating_indices.contains(&index) {
            state.inactivity_scores[index] -= u64::min(1, state.inactivity_scores[index]);
        } else {
            state.inactivity_scores[index] += context.inactivity_score_bias;
        }
        if not_is_leaking {
            state.inactivity_scores[index] -=
                u64::min(context.inactivity_score_recovery_rate, state.inactivity_scores[index]);
        }
    }
    Ok(())
}
pub fn process_rewards_and_penalties<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<()> {
    let current_epoch = get_current_epoch(state, context);
    if current_epoch == GENESIS_EPOCH {
        return Ok(())
    }
    let mut deltas = Vec::new();
    for flag_index in 0..PARTICIPATION_FLAG_WEIGHTS.len() {
        let flag_index_delta = get_flag_index_deltas(state, flag_index, context)?;
        deltas.push(flag_index_delta);
    }
    deltas.push(get_inactivity_penalty_deltas(state, context)?);
    for (rewards, penalties) in deltas {
        for index in 0..state.validators.len() {
            increase_balance(state, index, rewards[index]);
            decrease_balance(state, index, penalties[index]);
        }
    }
    Ok(())
}
pub fn process_participation_flag_updates<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
) -> Result<()> {
    let current_participation = mem::take(&mut state.current_epoch_participation);
    state.previous_epoch_participation = current_participation;
    let rotate_participation = vec![ParticipationFlags::default(); state.validators.len()];
    state.current_epoch_participation =
        rotate_participation.try_into().expect("should convert from Vec to List");
    Ok(())
}
pub fn process_sync_committee_updates<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<()> {
    let next_epoch = get_current_epoch(state, context) + 1;
    if next_epoch % context.epochs_per_sync_committee_period == 0 {
        let next_sync_committee = get_next_sync_committee(state, context)?;
        let current_sync_committee =
            mem::replace(&mut state.next_sync_committee, next_sync_committee);
        state.current_sync_committee = current_sync_committee;
    }
    Ok(())
}
pub fn process_registry_updates<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) {
    let current_epoch = get_current_epoch(state, context);
    for i in 0..state.validators.len() {
        let validator = &mut state.validators[i];
        if is_eligible_for_activation_queue(validator, context) {
            validator.activation_eligibility_epoch = current_epoch + 1;
        }
        if is_active_validator(validator, current_epoch) &&
            validator.effective_balance <= context.ejection_balance
        {
            initiate_validator_exit(state, i, context);
        }
    }
    let mut activation_queue =
        state
            .validators
            .iter()
            .enumerate()
            .filter_map(|(index, validator)| {
                if is_eligible_for_activation(state, validator) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<ValidatorIndex>>();
    activation_queue.sort_by(|&i, &j| {
        let a = &state.validators[i];
        let b = &state.validators[j];
        (a.activation_eligibility_epoch, i).cmp(&(b.activation_eligibility_epoch, j))
    });
    let activation_exit_epoch = compute_activation_exit_epoch(current_epoch, context);
    for i in activation_queue.into_iter().take(get_validator_churn_limit(state, context)) {
        let validator = &mut state.validators[i];
        validator.activation_epoch = activation_exit_epoch;
    }
}
pub fn process_eth1_data_reset<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) {
    let next_epoch = get_current_epoch(state, context) + 1;
    if next_epoch % context.epochs_per_eth1_voting_period == 0 {
        state.eth1_data_votes.clear();
    }
}
pub fn process_effective_balance_updates<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) {
    let hysteresis_increment = context.effective_balance_increment / context.hysteresis_quotient;
    let downward_threshold = hysteresis_increment * context.hysteresis_downward_multiplier;
    let upward_threshold = hysteresis_increment * context.hysteresis_upward_multiplier;
    for i in 0..state.validators.len() {
        let validator = &mut state.validators[i];
        let balance = state.balances[i];
        if balance + downward_threshold < validator.effective_balance ||
            validator.effective_balance + upward_threshold < balance
        {
            validator.effective_balance = Gwei::min(
                balance - balance % context.effective_balance_increment,
                context.max_effective_balance,
            );
        }
    }
}
pub fn process_slashings_reset<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) {
    let next_epoch = get_current_epoch(state, context) + 1;
    let slashings_index = next_epoch % context.epochs_per_slashings_vector;
    state.slashings[slashings_index as usize] = 0;
}
pub fn process_randao_mixes_reset<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) {
    let current_epoch = get_current_epoch(state, context);
    let next_epoch = current_epoch + 1;
    let mix_index = next_epoch % context.epochs_per_historical_vector;
    state.randao_mixes[mix_index as usize] = get_randao_mix(state, current_epoch).clone();
}
pub fn process_historical_roots_update<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<()> {
    let next_epoch = get_current_epoch(state, context) + 1;
    let epochs_per_historical_root = context.slots_per_historical_root / context.slots_per_epoch;
    if next_epoch % epochs_per_historical_root == 0 {
        let mut historical_batch = HistoricalSummary {
            block_summary_root: state.block_roots.hash_tree_root()?,
            state_summary_root: state.state_roots.hash_tree_root()?,
        };
        state.historical_roots.push(historical_batch.hash_tree_root()?)
    }
    Ok(())
}
pub fn weigh_justification_and_finalization<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    total_active_balance: Gwei,
    previous_epoch_target_balance: Gwei,
    current_epoch_target_balance: Gwei,
    context: &Context,
) -> Result<()> {
    let previous_epoch = get_previous_epoch(state, context);
    let current_epoch = get_current_epoch(state, context);
    let old_previous_justified_checkpoint = state.previous_justified_checkpoint.clone();
    let old_current_justified_checkpoint = state.current_justified_checkpoint.clone();
    state.previous_justified_checkpoint = state.current_justified_checkpoint.clone();
    state.justification_bits.copy_within(..JUSTIFICATION_BITS_LENGTH - 1, 1);
    state.justification_bits.set(0, false);
    if previous_epoch_target_balance * 3 >= total_active_balance * 2 {
        state.current_justified_checkpoint = Checkpoint {
            epoch: previous_epoch,
            root: *get_block_root(state, previous_epoch, context)?,
        };
        state.justification_bits.set(1, true);
    }
    if current_epoch_target_balance * 3 >= total_active_balance * 2 {
        state.current_justified_checkpoint = Checkpoint {
            epoch: current_epoch,
            root: *get_block_root(state, current_epoch, context)?,
        };
        state.justification_bits.set(0, true);
    }
    let bits = &state.justification_bits;
    if bits[1..4].all() && old_previous_justified_checkpoint.epoch + 3 == current_epoch {
        state.finalized_checkpoint = old_previous_justified_checkpoint.clone();
    }
    if bits[1..3].all() && old_previous_justified_checkpoint.epoch + 2 == current_epoch {
        state.finalized_checkpoint = old_previous_justified_checkpoint;
    }
    if bits[0..3].all() && old_current_justified_checkpoint.epoch + 2 == current_epoch {
        state.finalized_checkpoint = old_current_justified_checkpoint.clone();
    }
    if bits[0..2].all() && old_current_justified_checkpoint.epoch + 1 == current_epoch {
        state.finalized_checkpoint = old_current_justified_checkpoint;
    }
    Ok(())
}
pub fn get_proposer_reward<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    attesting_index: ValidatorIndex,
    context: &Context,
) -> Result<Gwei> {
    Ok(get_base_reward(state, attesting_index, context)? / context.proposer_reward_quotient)
}
pub fn get_finality_delay<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Epoch {
    get_previous_epoch(state, context) - state.finalized_checkpoint.epoch
}
pub fn is_in_inactivity_leak<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> bool {
    get_finality_delay(state, context) > context.min_epochs_to_inactivity_penalty
}
pub fn initialize_beacon_state_from_eth1<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
>(
    eth1_block_hash: Hash32,
    eth1_timestamp: u64,
    deposits: &mut [Deposit],
    execution_payload_header: Option<
        &ExecutionPayloadHeader<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>,
    >,
    context: &Context,
) -> Result<
    BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
> {
    let fork = Fork {
        previous_version: context.bellatrix_fork_version,
        current_version: context.bellatrix_fork_version,
        epoch: GENESIS_EPOCH,
    };
    let eth1_data = Eth1Data {
        block_hash: eth1_block_hash.clone(),
        deposit_count: deposits.len() as u64,
        ..Default::default()
    };
    let mut latest_block_body = BeaconBlockBody::<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >::default();
    let body_root = latest_block_body.hash_tree_root()?;
    let latest_block_header = BeaconBlockHeader { body_root, ..Default::default() };
    let randao_mixes = Vector::try_from(
        std::iter::repeat(eth1_block_hash)
            .take(context.epochs_per_historical_vector as usize)
            .collect::<Vec<_>>(),
    )
    .map_err(|(_, err)| err)?;
    let execution_payload_header = execution_payload_header.cloned().unwrap_or_default();
    let mut state = BeaconState {
        genesis_time: eth1_timestamp + context.genesis_delay,
        fork,
        eth1_data,
        latest_block_header,
        randao_mixes,
        latest_execution_payload_header: execution_payload_header,
        ..Default::default()
    };
    let mut leaves = List::<DepositData, DEPOSIT_DATA_LIST_BOUND>::default();
    for deposit in deposits.iter_mut() {
        leaves.push(deposit.data.clone());
        state.eth1_data.deposit_root = leaves.hash_tree_root()?;
        process_deposit(&mut state, deposit, context)?;
    }
    for i in 0..state.validators.len() {
        let validator = &mut state.validators[i];
        let balance = state.balances[i];
        let effective_balance = Gwei::min(
            balance - balance % context.effective_balance_increment,
            context.max_effective_balance,
        );
        validator.effective_balance = effective_balance;
        if validator.effective_balance == context.max_effective_balance {
            validator.activation_eligibility_epoch = GENESIS_EPOCH;
            validator.activation_epoch = GENESIS_EPOCH;
        }
    }
    state.genesis_validators_root = state.validators.hash_tree_root()?;
    let sync_committee = get_next_sync_committee(&state, context)?;
    state.current_sync_committee = sync_committee.clone();
    state.next_sync_committee = sync_committee;
    Ok(state)
}
pub fn is_valid_genesis_state<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> bool {
    if state.genesis_time < context.min_genesis_time {
        return false
    }
    if get_active_validator_indices(state, GENESIS_EPOCH).len() <
        context.min_genesis_active_validator_count
    {
        return false
    }
    true
}
pub fn get_genesis_block<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    genesis_state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
) -> Result<
    BeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
> {
    Ok(BeaconBlock { state_root: genesis_state.hash_tree_root()?, ..Default::default() })
}
pub fn get_inactivity_penalty_deltas<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<(Vec<Gwei>, Vec<Gwei>)> {
    let validator_count = state.validators.len();
    let rewards = vec![0; validator_count];
    let mut penalties = vec![0; validator_count];
    let previous_epoch = get_previous_epoch(state, context);
    let matching_target_indices = get_unslashed_participating_indices(
        state,
        TIMELY_TARGET_FLAG_INDEX,
        previous_epoch,
        context,
    )?;
    for i in get_eligible_validator_indices(state, context) {
        if !matching_target_indices.contains(&i) {
            let penalty_numerator =
                state.validators[i].effective_balance * state.inactivity_scores[i];
            let penalty_denominator =
                context.inactivity_score_bias * context.inactivity_penalty_quotient_bellatrix;
            penalties[i] += penalty_numerator / penalty_denominator;
        }
    }
    Ok((rewards, penalties))
}
pub fn slash_validator<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    slashed_index: ValidatorIndex,
    whistleblower_index: Option<ValidatorIndex>,
    context: &Context,
) -> Result<()> {
    let epoch = get_current_epoch(state, context);
    initiate_validator_exit(state, slashed_index, context);
    state.validators[slashed_index].slashed = true;
    state.validators[slashed_index].withdrawable_epoch = u64::max(
        state.validators[slashed_index].withdrawable_epoch,
        epoch + context.epochs_per_slashings_vector,
    );
    let slashings_index = epoch as usize % EPOCHS_PER_SLASHINGS_VECTOR;
    state.slashings[slashings_index] += state.validators[slashed_index].effective_balance;
    decrease_balance(
        state,
        slashed_index,
        state.validators[slashed_index].effective_balance /
            context.min_slashing_penalty_quotient_bellatrix,
    );
    let proposer_index = get_beacon_proposer_index(state, context)?;
    let whistleblower_index = whistleblower_index.unwrap_or(proposer_index);
    let whistleblower_reward =
        state.validators[slashed_index].effective_balance / context.whistleblower_reward_quotient;
    let proposer_reward_scaling_factor = PROPOSER_WEIGHT / WEIGHT_DENOMINATOR;
    let proposer_reward = whistleblower_reward * proposer_reward_scaling_factor;
    increase_balance(state, proposer_index, proposer_reward);
    increase_balance(state, whistleblower_index, whistleblower_reward - proposer_reward);
    Ok(())
}
pub fn is_merge_transition_complete<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
) -> bool {
    state.latest_execution_payload_header != ExecutionPayloadHeader::default()
}
pub fn is_merge_transition_block<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    body: &BeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
) -> bool {
    let transition_is_not_complete = !is_merge_transition_complete(state);
    let nonempty_payload = body.execution_payload != ExecutionPayload::default();
    transition_is_not_complete && nonempty_payload
}
pub fn is_execution_enabled<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    body: &BeaconBlockBody<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
) -> bool {
    let is_transition_block = is_merge_transition_block(state, body);
    let transition_is_complete = is_merge_transition_complete(state);
    is_transition_block || transition_is_complete
}
pub fn compute_timestamp_at_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    slot: Slot,
    context: &Context,
) -> Result<u64> {
    let slots_since_genesis = slot.checked_sub(GENESIS_SLOT).ok_or(Error::Underflow)?;
    let timestamp = state.genesis_time + slots_since_genesis * context.seconds_per_slot;
    Ok(timestamp)
}
pub fn add_flag(flags: ParticipationFlags, flag_index: usize) -> ParticipationFlags {
    let flag = 2u8.pow(flag_index as u32);
    flags | flag
}
pub fn has_flag(flags: ParticipationFlags, flag_index: usize) -> bool {
    let flag = 2u8.pow(flag_index as u32);
    flags & flag == flag
}
pub fn get_next_sync_committee_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<Vec<ValidatorIndex>> {
    let epoch = get_current_epoch(state, context) + 1;
    let max_random_byte = u8::MAX as u64;
    let active_validator_indices = get_active_validator_indices(state, epoch);
    let active_validator_count = active_validator_indices.len();
    let seed = get_seed(state, epoch, DomainType::SyncCommittee, context);
    let mut i: usize = 0;
    let mut sync_committee_indices = vec![];
    let mut hash_input = [0u8; 40];
    hash_input[..32].copy_from_slice(seed.as_ref());
    while sync_committee_indices.len() < context.sync_committee_size {
        let shuffled_index = compute_shuffled_index(
            i % active_validator_count,
            active_validator_count,
            &seed,
            context,
        )?;
        let candidate_index = active_validator_indices[shuffled_index];
        let i_bytes: [u8; 8] = ((i / 32) as u64).to_le_bytes();
        hash_input[32..].copy_from_slice(&i_bytes);
        let random_byte = hash(hash_input).as_ref()[i % 32] as u64;
        let effective_balance = state.validators[candidate_index].effective_balance;
        if effective_balance * max_random_byte >= context.max_effective_balance * random_byte {
            sync_committee_indices.push(candidate_index);
        }
        i += 1;
    }
    Ok(sync_committee_indices)
}
pub fn get_next_sync_committee<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<SyncCommittee<SYNC_COMMITTEE_SIZE>> {
    let indices = get_next_sync_committee_indices(state, context)?;
    let public_keys =
        indices.into_iter().map(|i| state.validators[i].public_key.clone()).collect::<Vec<_>>();
    let public_keys = Vector::<BlsPublicKey, SYNC_COMMITTEE_SIZE>::try_from(public_keys)
        .map_err(|(_, err)| err)?;
    let aggregate_public_key = eth_aggregate_public_keys(&public_keys)?;
    Ok(SyncCommittee::<SYNC_COMMITTEE_SIZE> { public_keys, aggregate_public_key })
}
pub fn get_base_reward_per_increment<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<Gwei> {
    Ok(context.effective_balance_increment * context.base_reward_factor /
        get_total_active_balance(state, context)?.integer_sqrt())
}
pub fn get_unslashed_participating_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    flag_index: usize,
    epoch: Epoch,
    context: &Context,
) -> Result<HashSet<ValidatorIndex>> {
    let previous_epoch = get_previous_epoch(state, context);
    let current_epoch = get_current_epoch(state, context);
    let is_current = epoch == current_epoch;
    if previous_epoch != epoch && current_epoch != epoch {
        return Err(Error::InvalidEpoch {
            requested: epoch,
            previous: previous_epoch,
            current: current_epoch,
        })
    }
    let epoch_participation = if is_current {
        &state.current_epoch_participation
    } else {
        &state.previous_epoch_participation
    };
    Ok(get_active_validator_indices(state, epoch)
        .into_iter()
        .filter(|&i| {
            let did_participate = has_flag(epoch_participation[i], flag_index);
            let not_slashed = !state.validators[i].slashed;
            did_participate && not_slashed
        })
        .collect::<HashSet<_>>())
}
pub fn get_attestation_participation_flag_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    data: &AttestationData,
    inclusion_delay: u64,
    context: &Context,
) -> Result<Vec<usize>> {
    let justified_checkpoint = if data.target.epoch == get_current_epoch(state, context) {
        &state.current_justified_checkpoint
    } else {
        &state.previous_justified_checkpoint
    };
    let is_matching_source = data.source == *justified_checkpoint;
    if !is_matching_source {
        return Err(invalid_operation_error(InvalidOperation::Attestation(
            InvalidAttestation::InvalidSource {
                expected: justified_checkpoint.clone(),
                source_checkpoint: data.source.clone(),
                current: get_current_epoch(state, context),
            },
        )))
    }
    let is_matching_target = is_matching_source &&
        (data.target.root == *get_block_root(state, data.target.epoch, context)?);
    let is_matching_head = is_matching_target &&
        (data.beacon_block_root == *get_block_root_at_slot(state, data.slot)?);
    let mut participation_flag_indices = Vec::new();
    if is_matching_source && inclusion_delay <= context.slots_per_epoch.integer_sqrt() {
        participation_flag_indices.push(TIMELY_SOURCE_FLAG_INDEX);
    }
    if is_matching_target && inclusion_delay <= context.slots_per_epoch {
        participation_flag_indices.push(TIMELY_TARGET_FLAG_INDEX);
    }
    if is_matching_head && inclusion_delay == context.min_attestation_inclusion_delay {
        participation_flag_indices.push(TIMELY_HEAD_FLAG_INDEX);
    }
    Ok(participation_flag_indices)
}
pub fn get_flag_index_deltas<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    flag_index: usize,
    context: &Context,
) -> Result<(Vec<Gwei>, Vec<Gwei>)> {
    let validator_count = state.validators.len();
    let mut rewards = vec![0; validator_count];
    let mut penalties = vec![0; validator_count];
    let previous_epoch = get_previous_epoch(state, context);
    let unslashed_participating_indices =
        get_unslashed_participating_indices(state, flag_index, previous_epoch, context)?;
    let weight = PARTICIPATION_FLAG_WEIGHTS[flag_index];
    let unslashed_participating_balance =
        get_total_balance(state, &unslashed_participating_indices, context)?;
    let unslashed_participating_increments =
        unslashed_participating_balance / context.effective_balance_increment;
    let active_increments =
        get_total_active_balance(state, context)? / context.effective_balance_increment;
    let not_leaking = !is_in_inactivity_leak(state, context);
    for index in get_eligible_validator_indices(state, context) {
        let base_reward = get_base_reward(state, index, context)?;
        if unslashed_participating_indices.contains(&index) {
            if not_leaking {
                let reward_numerator = base_reward * weight * unslashed_participating_increments;
                rewards[index] += reward_numerator / (active_increments * WEIGHT_DENOMINATOR);
            }
        } else if flag_index != TIMELY_HEAD_FLAG_INDEX {
            penalties[index] += base_reward * weight / WEIGHT_DENOMINATOR;
        }
    }
    Ok((rewards, penalties))
}
pub fn is_active_validator(validator: &Validator, epoch: Epoch) -> bool {
    validator.activation_epoch <= epoch && epoch < validator.exit_epoch
}
pub fn is_eligible_for_activation_queue(validator: &Validator, context: &Context) -> bool {
    validator.activation_eligibility_epoch == FAR_FUTURE_EPOCH &&
        validator.effective_balance == context.max_effective_balance
}
pub fn is_eligible_for_activation<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    validator: &Validator,
) -> bool {
    validator.activation_eligibility_epoch <= state.finalized_checkpoint.epoch &&
        validator.activation_epoch == FAR_FUTURE_EPOCH
}
pub fn is_slashable_validator(validator: &Validator, epoch: Epoch) -> bool {
    !validator.slashed &&
        validator.activation_epoch <= epoch &&
        epoch < validator.withdrawable_epoch
}
pub fn is_slashable_attestation_data(data_1: &AttestationData, data_2: &AttestationData) -> bool {
    let double_vote = data_1 != data_2 && data_1.target.epoch == data_2.target.epoch;
    let surround_vote =
        data_1.source.epoch < data_2.source.epoch && data_2.target.epoch < data_1.target.epoch;
    double_vote || surround_vote
}
pub fn is_valid_indexed_attestation<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    indexed_attestation: &mut IndexedAttestation<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<()> {
    let attesting_indices = &indexed_attestation.attesting_indices;
    if attesting_indices.is_empty() {
        return Err(invalid_operation_error(InvalidOperation::IndexedAttestation(
            InvalidIndexedAttestation::AttestingIndicesEmpty,
        )))
    }
    let mut prev = attesting_indices[0];
    let mut duplicates = HashSet::new();
    for &index in &attesting_indices[1..] {
        if index < prev {
            return Err(invalid_operation_error(InvalidOperation::IndexedAttestation(
                InvalidIndexedAttestation::AttestingIndicesNotSorted,
            )))
        }
        if index == prev {
            duplicates.insert(index);
        }
        prev = index;
    }
    if !duplicates.is_empty() {
        return Err(invalid_operation_error(InvalidOperation::IndexedAttestation(
            InvalidIndexedAttestation::DuplicateIndices(Vec::from_iter(duplicates)),
        )))
    }
    let mut public_keys = vec![];
    for &index in &attesting_indices[..] {
        let public_key = state.validators.get(index).map(|v| &v.public_key).ok_or_else(|| {
            invalid_operation_error(InvalidOperation::IndexedAttestation(
                InvalidIndexedAttestation::InvalidIndex(index),
            ))
        })?;
        public_keys.push(public_key);
    }
    let domain = get_domain(
        state,
        DomainType::BeaconAttester,
        Some(indexed_attestation.data.target.epoch),
        context,
    )?;
    let signing_root = compute_signing_root(&mut indexed_attestation.data, domain)?;
    fast_aggregate_verify(&public_keys, signing_root.as_ref(), &indexed_attestation.signature)
        .map_err(Into::into)
}
pub fn verify_block_signature<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    signed_block: &mut SignedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
    context: &Context,
) -> Result<()> {
    let proposer_index = signed_block.message.proposer_index;
    let proposer = state
        .validators
        .get(proposer_index)
        .ok_or(Error::OutOfBounds { requested: proposer_index, bound: state.validators.len() })?;
    let domain = get_domain(state, DomainType::BeaconProposer, None, context)?;
    let signing_root = compute_signing_root(&mut signed_block.message, domain)?;
    let public_key = &proposer.public_key;
    verify_signature(public_key, signing_root.as_ref(), &signed_block.signature).map_err(Into::into)
}
pub fn get_domain<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    domain_type: DomainType,
    epoch: Option<Epoch>,
    context: &Context,
) -> Result<Domain> {
    let epoch = epoch.unwrap_or_else(|| get_current_epoch(state, context));
    let fork_version = if epoch < state.fork.epoch {
        state.fork.previous_version
    } else {
        state.fork.current_version
    };
    compute_domain(domain_type, Some(fork_version), Some(state.genesis_validators_root), context)
}
pub fn get_current_epoch<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Epoch {
    compute_epoch_at_slot(state.slot, context)
}
pub fn compute_shuffled_index(
    mut index: usize,
    index_count: usize,
    seed: &Bytes32,
    context: &Context,
) -> Result<usize> {
    if index >= index_count {
        return Err(Error::InvalidShufflingIndex { index, total: index_count })
    }
    let mut pivot_input = [0u8; 33];
    pivot_input[..32].copy_from_slice(seed.as_ref());
    let mut source_input = [0u8; 37];
    source_input[..32].copy_from_slice(seed.as_ref());
    for current_round in 0..context.shuffle_round_count {
        pivot_input[32] = current_round as u8;
        let pivot_bytes: [u8; 8] = hash(pivot_input).as_ref()[..8].try_into().unwrap();
        let pivot = (u64::from_le_bytes(pivot_bytes) as usize) % index_count;
        let flip = (pivot + index_count - index) % index_count;
        let position = cmp::max(index, flip);
        let position_bytes: [u8; 4] = ((position / 256) as u32).to_le_bytes();
        source_input[32] = current_round as u8;
        source_input[33..].copy_from_slice(&position_bytes);
        let source = hash(source_input);
        let byte = source.as_ref()[(position % 256) / 8];
        let bit = (byte >> (position % 8)) % 2;
        index = if bit != 0 { flip } else { index };
    }
    Ok(index)
}
pub fn compute_proposer_index<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    indices: &[ValidatorIndex],
    seed: &Bytes32,
    context: &Context,
) -> Result<ValidatorIndex> {
    if indices.is_empty() {
        return Err(Error::CollectionCannotBeEmpty)
    }
    let max_byte = u8::MAX as u64;
    let mut i = 0;
    let total = indices.len();
    let mut hash_input = [0u8; 40];
    hash_input[..32].copy_from_slice(seed.as_ref());
    loop {
        let shuffled_index = compute_shuffled_index(i % total, total, seed, context)?;
        let candidate_index = indices[shuffled_index];
        let i_bytes: [u8; 8] = (i / 32).to_le_bytes();
        hash_input[32..].copy_from_slice(&i_bytes);
        let random_byte = hash(hash_input).as_ref()[i % 32] as u64;
        let effective_balance = state.validators[candidate_index].effective_balance;
        if effective_balance * max_byte >= context.max_effective_balance * random_byte {
            return Ok(candidate_index)
        }
        i += 1;
    }
}
pub fn compute_committee(
    indices: &[ValidatorIndex],
    seed: &Bytes32,
    index: usize,
    count: usize,
    context: &Context,
) -> Result<Vec<ValidatorIndex>> {
    let start = (indices.len() * index) / count;
    let end = (indices.len()) * (index + 1) / count;
    let mut committee = vec![0usize; end - start];
    for i in start..end {
        let index = compute_shuffled_index(i, indices.len(), seed, context)?;
        committee[i - start] = indices[index];
    }
    Ok(committee)
}
pub fn compute_epoch_at_slot(slot: Slot, context: &Context) -> Epoch {
    slot / context.slots_per_epoch
}
pub fn compute_start_slot_at_epoch(epoch: Epoch, context: &Context) -> Slot {
    epoch * context.slots_per_epoch
}
pub fn compute_activation_exit_epoch(epoch: Epoch, context: &Context) -> Epoch {
    epoch + 1 + context.max_seed_lookahead
}
pub fn compute_fork_digest(
    current_version: Version,
    genesis_validators_root: Root,
) -> Result<ForkDigest> {
    let fork_data_root = compute_fork_data_root(current_version, genesis_validators_root)?;
    let digest = &fork_data_root.as_ref()[..4];
    Ok(digest.try_into().expect("should not fail"))
}
pub fn compute_domain(
    domain_type: DomainType,
    fork_version: Option<Version>,
    genesis_validators_root: Option<Root>,
    context: &Context,
) -> Result<Domain> {
    let fork_version = fork_version.unwrap_or(context.genesis_fork_version);
    let genesis_validators_root = genesis_validators_root.unwrap_or_default();
    let fork_data_root = compute_fork_data_root(fork_version, genesis_validators_root)?;
    let mut domain = Domain::default();
    domain[..4].copy_from_slice(&domain_type.as_bytes());
    domain[4..].copy_from_slice(&fork_data_root.as_ref()[..28]);
    Ok(domain)
}
pub fn compute_fork_data_root(
    current_version: Version,
    genesis_validators_root: Root,
) -> Result<Root> {
    ForkData { current_version, genesis_validators_root }
        .hash_tree_root()
        .map_err(Error::Merkleization)
}
pub fn get_previous_epoch<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Epoch {
    let current_epoch = get_current_epoch(state, context);
    if current_epoch == GENESIS_EPOCH {
        GENESIS_EPOCH
    } else {
        current_epoch - 1
    }
}
pub fn get_block_root<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &'a BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    epoch: Epoch,
    context: &Context,
) -> Result<&'a Root> {
    get_block_root_at_slot(state, compute_start_slot_at_epoch(epoch, context))
}
pub fn get_block_root_at_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    slot: Slot,
) -> Result<&Root> {
    if slot >= state.slot || state.slot > (slot + SLOTS_PER_HISTORICAL_ROOT as Slot) {
        return Err(Error::SlotOutOfRange {
            requested: slot,
            lower_bound: state.slot - 1,
            upper_bound: state.slot + SLOTS_PER_HISTORICAL_ROOT as Slot,
        })
    }
    Ok(&state.block_roots[slot as usize % SLOTS_PER_HISTORICAL_ROOT])
}
pub fn get_randao_mix<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    epoch: Epoch,
) -> &Bytes32 {
    let epoch = epoch as usize % EPOCHS_PER_HISTORICAL_VECTOR;
    &state.randao_mixes[epoch]
}
pub fn get_active_validator_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    epoch: Epoch,
) -> Vec<ValidatorIndex> {
    let mut active = Vec::with_capacity(state.validators.len());
    for (i, v) in state.validators.iter().enumerate() {
        if is_active_validator(v, epoch) {
            active.push(i)
        }
    }
    active
}
pub fn get_validator_churn_limit<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> usize {
    let active_validator_indices =
        get_active_validator_indices(state, get_current_epoch(state, context));
    u64::max(
        context.min_per_epoch_churn_limit,
        active_validator_indices.len() as u64 / context.churn_limit_quotient,
    ) as usize
}
pub fn get_seed<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    epoch: Epoch,
    domain_type: DomainType,
    context: &Context,
) -> Bytes32 {
    let mix_epoch = epoch + (context.epochs_per_historical_vector - context.min_seed_lookahead) - 1;
    let mix = get_randao_mix(state, mix_epoch);
    let mut input = [0u8; 44];
    input[..4].copy_from_slice(&domain_type.as_bytes());
    input[4..12].copy_from_slice(&epoch.to_le_bytes());
    input[12..].copy_from_slice(mix.as_ref());
    hash(input)
}
pub fn get_committee_count_per_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    epoch: Epoch,
    context: &Context,
) -> usize {
    u64::max(
        1,
        u64::min(
            context.max_committees_per_slot,
            get_active_validator_indices(state, epoch).len() as u64 /
                context.slots_per_epoch /
                context.target_committee_size,
        ),
    ) as usize
}
pub fn get_beacon_committee<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    slot: Slot,
    index: CommitteeIndex,
    context: &Context,
) -> Result<Vec<ValidatorIndex>> {
    let epoch = compute_epoch_at_slot(slot, context);
    let committees_per_slot = get_committee_count_per_slot(state, epoch, context);
    let indices = get_active_validator_indices(state, epoch);
    let seed = get_seed(state, epoch, DomainType::BeaconAttester, context);
    let index = (slot % context.slots_per_epoch) * committees_per_slot as u64 + index as u64;
    let count = committees_per_slot as u64 * context.slots_per_epoch;
    compute_committee(&indices, &seed, index as usize, count as usize, context)
}
pub fn get_beacon_proposer_index<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<ValidatorIndex> {
    let epoch = get_current_epoch(state, context);
    let mut input = [0u8; 40];
    input[..32]
        .copy_from_slice(get_seed(state, epoch, DomainType::BeaconProposer, context).as_ref());
    input[32..40].copy_from_slice(&state.slot.to_le_bytes());
    let seed = hash(input);
    let indices = get_active_validator_indices(state, epoch);
    compute_proposer_index(state, &indices, &seed, context)
}
pub fn get_total_balance<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    indices: &HashSet<ValidatorIndex>,
    context: &Context,
) -> Result<Gwei> {
    let total_balance = indices
        .iter()
        .try_fold(Gwei::default(), |acc, i| acc.checked_add(state.validators[*i].effective_balance))
        .ok_or(Error::Overflow)?;
    Ok(u64::max(total_balance, context.effective_balance_increment))
}
pub fn get_total_active_balance<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<Gwei> {
    let indices = get_active_validator_indices(state, get_current_epoch(state, context));
    get_total_balance(state, &HashSet::from_iter(indices), context)
}
pub fn get_indexed_attestation<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    attestation: &Attestation<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<IndexedAttestation<MAX_VALIDATORS_PER_COMMITTEE>> {
    let bits = &attestation.aggregation_bits;
    let mut attesting_indices = get_attesting_indices(state, &attestation.data, bits, context)?
        .into_iter()
        .collect::<Vec<_>>();
    attesting_indices.sort_unstable();
    let attesting_indices = attesting_indices.try_into().map_err(|(_, err)| err)?;
    Ok(IndexedAttestation {
        attesting_indices,
        data: attestation.data.clone(),
        signature: attestation.signature.clone(),
    })
}
pub fn get_attesting_indices<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    data: &AttestationData,
    bits: &Bitlist<MAX_VALIDATORS_PER_COMMITTEE>,
    context: &Context,
) -> Result<HashSet<ValidatorIndex>> {
    let committee = get_beacon_committee(state, data.slot, data.index, context)?;
    if bits.len() != committee.len() {
        return Err(invalid_operation_error(InvalidOperation::Attestation(
            InvalidAttestation::Bitfield { expected_length: committee.len(), length: bits.len() },
        )))
    }
    let mut indices = HashSet::with_capacity(bits.capacity());
    for (i, validator_index) in committee.iter().enumerate() {
        if bits[i] {
            indices.insert(*validator_index);
        }
    }
    Ok(indices)
}
pub fn increase_balance<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    index: ValidatorIndex,
    delta: Gwei,
) {
    state.balances[index] += delta;
}
pub fn decrease_balance<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    index: ValidatorIndex,
    delta: Gwei,
) {
    if delta > state.balances[index] {
        state.balances[index] = 0
    } else {
        state.balances[index] -= delta
    }
}
pub fn initiate_validator_exit<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    index: ValidatorIndex,
    context: &Context,
) {
    if state.validators[index].exit_epoch != FAR_FUTURE_EPOCH {
        return
    }
    let mut exit_epochs: Vec<Epoch> = state
        .validators
        .iter()
        .filter(|v| v.exit_epoch != FAR_FUTURE_EPOCH)
        .map(|v| v.exit_epoch)
        .collect();
    exit_epochs.push(compute_activation_exit_epoch(get_current_epoch(state, context), context));
    let mut exit_queue_epoch = *exit_epochs.iter().max().unwrap();
    let exit_queue_churn =
        state.validators.iter().filter(|v| v.exit_epoch == exit_queue_epoch).count();
    if exit_queue_churn >= get_validator_churn_limit(state, context) {
        exit_queue_epoch += 1;
    }
    state.validators[index].exit_epoch = exit_queue_epoch;
    state.validators[index].withdrawable_epoch =
        state.validators[index].exit_epoch + context.min_validator_withdrawability_delay;
}
pub fn get_eligible_validator_indices<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &'a BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> impl Iterator<Item = ValidatorIndex> + 'a {
    let previous_epoch = get_previous_epoch(state, context);
    state.validators.iter().enumerate().filter_map(move |(i, validator)| {
        if is_active_validator(validator, previous_epoch) ||
            (validator.slashed && previous_epoch + 1 < validator.withdrawable_epoch)
        {
            Some(i)
        } else {
            None
        }
    })
}
pub fn process_slots<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    slot: Slot,
    context: &Context,
) -> Result<()> {
    if state.slot >= slot {
        return Err(Error::TransitionToPreviousSlot { requested: slot, current: state.slot })
    }
    while state.slot < slot {
        process_slot(state, context)?;
        if (state.slot + 1) % context.slots_per_epoch == 0 {
            process_epoch(state, context)?;
        }
        state.slot += 1;
    }
    Ok(())
}
pub fn process_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    context: &Context,
) -> Result<()> {
    let previous_state_root = state.hash_tree_root()?;
    let root_index = state.slot % context.slots_per_historical_root;
    state.state_roots[root_index as usize] = previous_state_root;
    if state.latest_block_header.state_root == Root::default() {
        state.latest_block_header.state_root = previous_state_root;
    }
    let previous_block_root = state.latest_block_header.hash_tree_root()?;
    let root_index = state.slot % context.slots_per_historical_root;
    state.block_roots[root_index as usize] = previous_block_root;
    Ok(())
}
pub fn state_transition_block_in_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    signed_block: &mut SignedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
    execution_engine: E,
    validation: Validation,
    context: &Context,
) -> Result<()> {
    let validate_result = match validation {
        Validation::Enabled => true,
        Validation::Disabled => false,
    };
    if validate_result {
        verify_block_signature(state, signed_block, context)?;
    }
    let block = &mut signed_block.message;
    process_block(state, block, execution_engine, context)?;
    if validate_result && block.state_root != state.hash_tree_root()? {
        Err(Error::InvalidStateRoot)
    } else {
        Ok(())
    }
}
pub fn state_transition<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_PROPOSER_SLASHINGS: usize,
    const MAX_ATTESTER_SLASHINGS: usize,
    const MAX_ATTESTATIONS: usize,
    const MAX_DEPOSITS: usize,
    const MAX_VOLUNTARY_EXITS: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    const MAX_BLS_TO_EXECUTION_CHANGES: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    signed_block: &mut SignedBeaconBlock<
        MAX_PROPOSER_SLASHINGS,
        MAX_VALIDATORS_PER_COMMITTEE,
        MAX_ATTESTER_SLASHINGS,
        MAX_ATTESTATIONS,
        MAX_DEPOSITS,
        MAX_VOLUNTARY_EXITS,
        SYNC_COMMITTEE_SIZE,
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        MAX_BLS_TO_EXECUTION_CHANGES,
    >,
    execution_engine: E,
    validation: Validation,
    context: &Context,
) -> Result<()> {
    process_slots(state, signed_block.message.slot, context)?;
    state_transition_block_in_slot(state, signed_block, execution_engine, validation, context)
}
