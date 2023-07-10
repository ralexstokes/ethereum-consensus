mod config;

use crate::api_client::Client;

use config::{BeaconMethod, Namespace::Beacon};
pub use config::CliConfig;

pub async fn run_cli(client: &Client, args: &CliConfig) {
    match args.command {
        Beacon(BeaconMethod::Genesis(genesis)) => genesis.execute(&client).await,
        Beacon(BeaconMethod::Root(ref root_arg)) => root_arg.execute(&client).await,
    }
}
