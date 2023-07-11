mod config;

use crate::api_client::Client;

pub use config::CliConfig;
use config::{BeaconMethod, Namespace::Beacon};
use ethereum_consensus::primitives::Root;

pub async fn run_cli(client: &Client, args: &CliConfig) {
    match &args.namespace {
        Beacon(BeaconMethod::Genesis) => {
            println!("{:?}", &client.get_genesis_details().await.unwrap());
        }
        Beacon(BeaconMethod::Root(arg)) => {
            println!("{}", &client.get_state_root(arg.state_id.clone()).await.unwrap())
        }
        _ => println!("not yet implemented"),
    }
}
