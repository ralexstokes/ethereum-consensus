pub mod api_client;
use url::Url;
pub mod cli;
pub mod error;
pub mod serde;
pub mod types;
use clap::Parser;

#[tokio::main]
async fn main() {
    // read in args from CLI
    let args = cli::CliConfig::parse();
    // instantiate client and pass to run_cli
    let url: Url = Url::parse(&args.endpoint).unwrap();
    let client = api_client::Client::new(url);
    cli::run_cli(client, args).await;
}
