use bip32::Mnemonic;
use clap::{Args, Subcommand};
use rand_core::OsRng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub fn generate_random_mnemonic() -> Result<Mnemonic, Error> {
    Ok(Mnemonic::random(&mut OsRng, Default::default()))
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Mnemonic,
}

#[derive(Debug, Args)]
#[clap(about = "utilities for managing validators")]
pub struct Command {
    #[clap(subcommand)]
    pub command: Commands,
}

impl Command {
    pub fn execute(self) -> eyre::Result<()> {
        match self.command {
            Commands::Mnemonic => {
                let mnemonic = generate_random_mnemonic()?;
                println!("{}", mnemonic.phrase());
                Ok(())
            }
        }
    }
}
