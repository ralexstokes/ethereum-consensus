use beacon_api_client::{mainnet::Client, StateId};
use url::Url;

#[tokio::main]
async fn main() {
    let s = "http://127.0.0.1:8003/";
    let url: Url = Url::parse(s).unwrap();
    let client = Client::new(url);

    let checkpoints = client.get_finality_checkpoints(StateId::Finalized).await.unwrap();

    println!("previous checkpoint: {:?}", checkpoints.previous_justified);
    println!("current checkpoint: {:?}", checkpoints.current_justified);
    println!("finalized checkpoint: {:?}", checkpoints.finalized);
}
