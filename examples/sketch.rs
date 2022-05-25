use ethereum_consensus::altair::mainnet::{self as altair};
use ethereum_consensus::bellatrix::mainnet as bellatrix;
use ethereum_consensus::phase0::mainnet::{self as phase0};
use ethereum_consensus::state_transition::{Context, Validation};

fn main() {
    let context = Context::for_mainnet();
    let mut state = phase0::BeaconState::default();
    let mut signed_block = phase0::SignedBeaconBlock::default();
    signed_block.message.slot = 1;

    let previous_epoch = phase0::get_previous_epoch(&state, &context);
    dbg!(previous_epoch);

    let _ = phase0::state_transition(&mut state, &mut signed_block, Validation::Enabled, &context);
    dbg!(state.fork);

    let mut state = altair::BeaconState::default();
    let mut signed_block = altair::SignedBeaconBlock::default();
    state.slot = 32;
    signed_block.message.slot = 33;

    let current_epoch = altair::get_current_epoch(&state, &context);
    dbg!(current_epoch);

    let _ = altair::state_transition(&mut state, &mut signed_block, Validation::Enabled, &context);
    dbg!(state.fork);

    dbg!(altair::SyncCommittee::default());
    dbg!(bellatrix::ExecutionPayload::default());
    dbg!(bellatrix::ExecutionPayloadHeader::default());
}
