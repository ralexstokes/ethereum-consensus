use beacon_api_client::mainnet::Client;
use ethereum_consensus::builder::SignedValidatorRegistration;
use url::Url;

#[tokio::main]
async fn main() {
    let client = Client::new(Url::parse("http://localhost:8080").unwrap());
    let data = SignedValidatorRegistration::default();
    let response = client.http_post("/eth/v1/builder/validators", &data).await.unwrap();
    dbg!(&response);
    dbg!(&response.status());
    dbg!(&response.text().await);
}
