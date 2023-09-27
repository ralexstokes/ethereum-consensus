mod keys;
mod mnemonic;

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Mnemonic,
    GenerateKeystores { phrase: String, start: u32, end: u32 },
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
                let mnemonic = mnemonic::generate_random_from_system_entropy()?;
                println!("{}", mnemonic);
                Ok(())
            }
            Commands::GenerateKeystores { phrase, start, end } => {
                let mnemonic = mnemonic::recover_from_phrase(&phrase)?;
                let seed = mnemonic::to_seed(mnemonic, None);
                let keys = keys::generate(&seed, start, end);
                dbg!(keys);
                Ok(())
            }
        }
    }
}
