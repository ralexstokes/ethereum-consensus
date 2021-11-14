use ethereum_consensus::altair::light_client::Snapshot;
use ethereum_consensus::phase0::mainnet::{self, apply_block, BeaconState, SignedBeaconBlock};

fn main() {
    let context = mainnet::context();
    let state = BeaconState::default();
    let signed_block = SignedBeaconBlock::default();
    let post = apply_block(state, signed_block, &context);
    dbg!(post);

    let snapshot = Snapshot::default();
    dbg!(snapshot);
}
