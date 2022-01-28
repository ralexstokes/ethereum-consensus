use ethereum_consensus::altair::light_client::Snapshot;
use ethereum_consensus::phase0::mainnet::{self, BeaconState, Context, SignedBeaconBlock};

fn main() {
    let context = Context::for_mainnet();
    let mut state = BeaconState::default();
    let mut signed_block = SignedBeaconBlock::default();

    let previous_epoch = mainnet::get_previous_epoch(&state, &context);
    dbg!(previous_epoch);

    mainnet::apply_block(&mut state, &mut signed_block, &context).expect("can transition");
    dbg!(state.fork);

    let snapshot = Snapshot::default();
    dbg!(snapshot.current_sync_committee.aggregate_pubkey);
}
