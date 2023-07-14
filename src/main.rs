use beacon_api_client::{mainnet::Client, run_cli, CliConfig};
use clap::Parser;
use url::Url;

#[tokio::main]
async fn main() {
    let args = CliConfig::parse();
    let url = Url::parse(&args.endpoint).unwrap();
    let client = Client::new(url);
    run_cli(&client, &args).await;
}
