// use crate::altair::beacon_state::BeaconState;
// use crate::altair::state_transition::Context;
// use crate::phase0::beacon_block::BeaconBlock;
// use crate::phase0::state_transition::block_processing::{
//     process_block_header, process_eth1_data, process_operations, process_randao,
// };
// use crate::phase0::state_transition::Error;

/*
    With `process_block` importing from `phase0`, its fns (process_block_header, etc.) all expect types
    from phase0 and not altair. The preference is to reduce any code duplication within the altair module
    but tbd on the resolution path.
*/

// pub fn process_block<
//     const SLOTS_PER_HISTORICAL_ROOT: usize,
//     const HISTORICAL_ROOTS_LIMIT: usize,
//     const ETH1_DATA_VOTES_BOUND: usize,
//     const VALIDATOR_REGISTRY_LIMIT: usize,
//     const EPOCHS_PER_HISTORICAL_VECTOR: usize,
//     const EPOCHS_PER_SLASHINGS_VECTOR: usize,
//     const MAX_VALIDATORS_PER_COMMITTEE: usize,
//     const PENDING_ATTESTATIONS_BOUND: usize,
//     const MAX_PROPOSER_SLASHINGS: usize,
//     const MAX_ATTESTER_SLASHINGS: usize,
//     const MAX_ATTESTATIONS: usize,
//     const MAX_DEPOSITS: usize,
//     const MAX_VOLUNTARY_EXITS: usize,
// >(
//     state: &mut BeaconState<
//         SLOTS_PER_HISTORICAL_ROOT,
//         HISTORICAL_ROOTS_LIMIT,
//         ETH1_DATA_VOTES_BOUND,
//         VALIDATOR_REGISTRY_LIMIT,
//         EPOCHS_PER_HISTORICAL_VECTOR,
//         EPOCHS_PER_SLASHINGS_VECTOR,
//         MAX_VALIDATORS_PER_COMMITTEE,
//         PENDING_ATTESTATIONS_BOUND,
//     >,
//     block: &mut BeaconBlock<
//         MAX_PROPOSER_SLASHINGS,
//         MAX_VALIDATORS_PER_COMMITTEE,
//         MAX_ATTESTER_SLASHINGS,
//         MAX_ATTESTATIONS,
//         MAX_DEPOSITS,
//         MAX_VOLUNTARY_EXITS,
//     >,
//     context: &Context,
// ) -> Result<(), Error> {
//     process_block_header(state, block, context)?;
//     process_randao(state, &block.body, context)?;
//     process_eth1_data(state, &block.body, context);
//     process_operations(state, &mut block.body, context)?;
//     Ok(())
// }
