mod config;


use crate::api_client::Client;

use config::{BeaconMethod, Namespace::Beacon};
pub use config::CliConfig;

pub async fn run_cli(client: &Client, args: &CliConfig) {
    match args.command {
        Beacon(BeaconMethod::Genesis(arg)) => arg.execute(client).await,
        Beacon(BeaconMethod::Root(arg)) => arg.execute(client).await,
    }
}
