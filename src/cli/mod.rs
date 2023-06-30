mod config;
use crate::api_client;

pub use config::{BeaconMethod, CliConfig, Namespace::Beacon};

pub async fn run_cli(client: api_client::Client, args: CliConfig) {
    match args.command {
        Beacon(BeaconMethod::Genesis(genesis)) => genesis.execute(&client).await,
        Beacon(BeaconMethod::Root(ref root_arg)) => root_arg.execute(&client).await,
    }
}
