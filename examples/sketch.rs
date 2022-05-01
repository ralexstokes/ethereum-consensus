use ethereum_consensus::altair::mainnet::SyncCommittee;
use ethereum_consensus::bellatrix::mainnet::{ExecutionPayload, ExecutionPayloadHeader};
use ethereum_consensus::phase0::mainnet::{self, BeaconState, Context, SignedBeaconBlock};

fn main() {
    let context = Context::for_mainnet();
    let mut state = BeaconState::default();
    let mut signed_block = SignedBeaconBlock::default();
    signed_block.message.slot = 1;

    let previous_epoch = mainnet::get_previous_epoch(&state, &context);
    dbg!(previous_epoch);

    let _ = mainnet::apply_block(&mut state, &mut signed_block, &context);
    dbg!(state.fork);

    dbg!(SyncCommittee::default());
    dbg!(ExecutionPayload::default());
    dbg!(ExecutionPayloadHeader::default());
}
