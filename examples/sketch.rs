use beacon_api_client::{ApiError, ApiResult, ConsensusVersion, Value, VersionedValue};
use ethereum_consensus::bellatrix::mainnet::BlindedBeaconBlock;
use serde_json;

fn main() {
    let block = Value {
        data: BlindedBeaconBlock::default(),
    };
    let block_repr = serde_json::to_string(&block).unwrap();
    println!("{block_repr}");

    let block_with_version = VersionedValue {
        version: ConsensusVersion::Bellatrix,
        data: BlindedBeaconBlock::default(),
    };
    let block_with_version_repr = serde_json::to_string(&block_with_version).unwrap();
    println!("{block_with_version_repr}");

    let full_success_response = ApiResult::Ok(block_with_version);
    let str_repr = serde_json::to_string(&full_success_response).unwrap();
    println!("{}", str_repr);

    let recovered_success: ApiResult<VersionedValue<BlindedBeaconBlock>> =
        serde_json::from_str(&str_repr).unwrap();
    println!("{:#?}", recovered_success);

    let full_error_response: ApiResult<Value<BlindedBeaconBlock>> =
        ApiResult::Err(ApiError::from((404, "some failure")));
    let str_repr = serde_json::to_string(&full_error_response).unwrap();
    println!("{str_repr}");

    let recovered_error: ApiResult<String> = serde_json::from_str(&str_repr).unwrap();
    println!("{:#?}", recovered_error);
}
