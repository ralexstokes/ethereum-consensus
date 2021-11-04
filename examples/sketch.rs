use ethereum_consensus::altair::light_client::Snapshot;
use ethereum_consensus::phase0::mainnet::{apply_block, BeaconState, SignedBeaconBlock};

fn main() {
    let state = BeaconState::default();
    let signed_block = SignedBeaconBlock::default();
    let post = apply_block(state, signed_block);
    dbg!(post);

    let snapshot = Snapshot::default();
    dbg!(snapshot);
}
