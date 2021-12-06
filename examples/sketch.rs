use ethereum_consensus::altair::light_client::Snapshot;
use ethereum_consensus::phase0::mainnet::{self, BeaconState, Context, SignedBeaconBlock};

fn main() {
    let context = Context::for_mainnet();
    let state = BeaconState::default();
    let signed_block = SignedBeaconBlock::default();
    let previous_epoch = mainnet::get_previous_epoch(&state, &context);
    dbg!(previous_epoch);
    let post = mainnet::apply_block(&state, &signed_block, &context);
    dbg!(post.fork);

    let snapshot = Snapshot::default();
    dbg!(snapshot.current_sync_committee.aggregate_pubkey);
}
