use url::Url;
use clap::Parser;
use beacon_api_client::{Client, CliConfig, run_cli};

#[tokio::main]
async fn main() {
    let args = CliConfig::parse();
    let url = Url::parse(&args.endpoint).unwrap();
    let client = Client::new(url);
    run_cli(&client, &args).await;
}
