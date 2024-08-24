use beacon_api_client::{mainnet::Client, StateId};
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse(&*std::env::var("BEACON_URL").unwrap()).unwrap();
    // let url = Url::parse("http://localhost:5052/").unwrap();
    let client = Client::new(url);
    let state = client.get_state_raw(StateId::Slot(9568224)).await.unwrap();
    // run_cli(&client, &args).await;
}
