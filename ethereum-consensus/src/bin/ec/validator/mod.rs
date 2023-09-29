mod keys;
mod keystores;
mod mnemonic;

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Mnemonic,
    #[clap(
        about = "Generates keystores (with passphrases) that target a format compatible with `lighthouse validator-manager` utility."
    )]
    GenerateLighthouseKeystores {
        #[clap(help = "BIP-39 mnemonic to use following EIP-2334")]
        phrase: String,
        #[clap(help = "EIP-2334 index to start key generation (inclusive)")]
        start: u32,
        #[clap(help = "EIP-2334 index to stop key generation (exclusive)")]
        end: u32,
    },
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
            Commands::GenerateLighthouseKeystores { phrase, start, end } => {
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
