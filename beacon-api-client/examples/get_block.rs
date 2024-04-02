use beacon_api_client::{mainnet::Client, BlockId};
use ethereum_consensus::primitives::Root;
use hex::FromHex;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("http://localhost:5052").unwrap();
    let client = Client::new(url);

    let root_hex = "421c16805c3416150aa04533fdfe7fc3f0880d0ed86cee33fa58011f10dd95c8";
    let root = Root::from_hex(root_hex).unwrap();
    let id = BlockId::Root(root);

    let block = client.get_beacon_block(id).await.unwrap();
    dbg!(block);
}
