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
pub struct StateIdArg {
    pub state_id: StateId,
}
