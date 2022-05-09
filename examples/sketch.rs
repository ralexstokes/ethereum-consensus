use beacon_api_client::{ApiResponse, ConsensusVersion, VersionedApiResponse};
use ethereum_consensus::bellatrix::mainnet::BlindedBeaconBlock;
use serde_json;

fn main() {
    let block: ApiResponse<BlindedBeaconBlock> = ApiResponse {
        data: BlindedBeaconBlock::default(),
    };
    let block_repr = serde_json::to_string(&block).unwrap();
    println!("{block_repr}");

    let block_with_version: VersionedApiResponse<BlindedBeaconBlock> = VersionedApiResponse {
        version: ConsensusVersion::Bellatrix,
        data: BlindedBeaconBlock::default(),
    };
    let block_with_version_repr = serde_json::to_string(&block_with_version).unwrap();
    println!("{block_with_version_repr}");
}
