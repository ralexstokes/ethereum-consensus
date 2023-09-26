use bip32::Mnemonic;
use clap::{Args, Subcommand};
use rand_core::OsRng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub fn generate_random_mnemonic() -> Result<Mnemonic, Error> {
    Ok(Mnemonic::random(OsRng, Default::default()))
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Mnemonic,
    GenerateKeystores { phrase: String, start: usize, end: usize },
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
                println!("generated new mnemonic from system entropy and **empty** password");
                println!("{}", mnemonic.phrase());
                Ok(())
            }
            Commands::GenerateKeystores { phrase, start, end } => {
                println!("recovering mnemonic from phrase (with empty password)");
                let mnemonic = Mnemonic::new(phrase, Default::default())?;
                let seed = mnemonic.to_seed("");
                println!("generating key stores for key indices from {start:?} to {end}");
                Ok(())
            }
        }
    }
}
