use beacon_api_client::{mainnet::Client, BlockId};
use url::Url;

#[tokio::main]
async fn main() {
    let username = dotenv::var("NODE_USERNAME").unwrap();
    let password = dotenv::var("NODE_PASSWORD").unwrap();
    let devnet_name = "dencun-devnet-8";
    let cl = "teku";
    let el = "geth";

    let url_str =
        format!("https://{username}:{password}@bn.{cl}-{el}-1.srv.{devnet_name}.ethpandaops.io",);

    let url = Url::parse(&url_str).unwrap();
    let client = Client::new(url);

    let id = BlockId::Slot(194496);
    let indices = [];

    let blobs = client.get_blob_sidecars(id, &indices).await.unwrap();
    dbg!(blobs);
}
