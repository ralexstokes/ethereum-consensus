use crate::{api_client::Client, types::StateId};
use clap::{Args, Parser, Subcommand};
use std::fmt;

#[derive(Debug, Parser)]
#[clap(version, about = "Beacon API client")]
pub struct CliConfig {
    #[clap(long)]
    pub endpoint: String,
    #[clap(subcommand)]
    pub namespace: Namespace,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Namespace {
    #[clap(subcommand)]
    Beacon(BeaconMethod),
}


#[derive(Debug, Clone, Subcommand)]
pub enum BeaconMethod {
    Genesis,
    Root(StateIdArg),
}

#[derive(Debug, Clone, Args)]
pub struct GenesisArg {
    genesis: Option<StateId>,
}

impl GenesisArg {
    pub async fn execute(&self, client: &Client) {
        let out = client.get_genesis_details().await.unwrap();
        println!("{:?}", out);
    }
}

#[derive(Debug, Clone, Args)]
pub struct RootArg {
    pub state_id: StateId,
}

impl RootArg {
    pub async fn execute(&self, client: &Client) {
        let id = &self.state_id;
        let out = client.get_state_root(id.to_owned()).await.unwrap();
        println!("{}", out);
    }
}
