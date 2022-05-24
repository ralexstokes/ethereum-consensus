use ethereum_consensus::altair::mainnet::SyncCommittee;
use ethereum_consensus::bellatrix::mainnet::{ExecutionPayload, ExecutionPayloadHeader};
use ethereum_consensus::phase0::mainnet::{self as spec, BeaconState, SignedBeaconBlock};
use ethereum_consensus::state_transition::{Context, Validation};

fn main() {
    let context = Context::for_mainnet();
    let mut state = BeaconState::default();
    let mut signed_block = SignedBeaconBlock::default();
    signed_block.message.slot = 1;

    let previous_epoch = spec::get_previous_epoch(&state, &context);
    dbg!(previous_epoch);

    let _ = spec::state_transition(&mut state, &mut signed_block, Validation::Enabled, &context);
    dbg!(state.fork);

    dbg!(SyncCommittee::default());
    dbg!(ExecutionPayload::default());
    dbg!(ExecutionPayloadHeader::default());
}
