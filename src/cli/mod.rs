mod config;

use crate::api_client::Client;

pub use config::CliConfig;
use config::{BeaconMethod, Namespace::Beacon};

pub async fn run_cli(client: &Client, args: &CliConfig) {
    match args.namespace {
        Beacon(BeaconMethod::Genesis) => {
            println!("{:?}", client.get_genesis_details().await.unwrap())
        }
        _ => println!("not yet implemented"),
        //Beacon(BeaconMethod::Root(arg)) => arg.execute(client).await,
    }
}
