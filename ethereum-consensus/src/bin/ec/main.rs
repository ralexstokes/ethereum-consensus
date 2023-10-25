mod bls;
mod validator;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Validator(validator::Command),
    Bls(bls::Command),
}

#[derive(Debug, Parser)]
#[clap(author, version, about = "utilities for ethereum consensus", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validator(cmd) => cmd.execute(),
        Commands::Bls(cmd) => cmd.execute(),
    }
}
