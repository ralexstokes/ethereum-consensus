mod bip39;
mod keystores;

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
                let mnemonic = bip39::generate_random_mnemonic()?;
                println!("{}", mnemonic.to_string());
                Ok(())
            }
            Commands::GenerateKeystores { phrase, start, end } => {
                let mnemonic = bip39::recover_mnemonic_from_phrase(&phrase)?;
                let keystores = keystores::generate(mnemonic, start, end)?;
                dbg!(keystores);
                Ok(())
            }
        }
    }
}
