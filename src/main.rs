use beacon_api_client::Client;
use beacon_api_client::StateId;
use url::Url;

#[tokio::main]
async fn main() {
    let s = "http://127.0.0.1:8003/";
    let url: Url = Url::parse(s).unwrap();
    let client = Client::new(url);

    let root = client.get_state_root(StateId::Finalized).await;
    let fork = client.get_fork(StateId::Finalized).await;

    // unwrap fork and extract fields
    let fk = fork.unwrap();
    let pv = fk.previous_version;
    let cv = fk.current_version;
    let ep = fk.epoch;

    println!("\nroot:\n{:?}\n", root.unwrap());
    println!("fork.previous_version = {:?}", pv);
    println!("fork.current_version = {:?}", cv);
    println!("fork.epoch = {:?}", ep);
}
