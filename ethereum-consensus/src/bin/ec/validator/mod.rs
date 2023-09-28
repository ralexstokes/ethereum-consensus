mod keys;
mod keystores;
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
                let (signing_keys, _withdrawal_keys) = keys::generate(&seed, start, end);
                let keystores_with_passphrases = keystores::generate(signing_keys);
                println!("{}", serde_json::to_string_pretty(&keystores_with_passphrases).unwrap());
                Ok(())
            }
        }
    }
}
