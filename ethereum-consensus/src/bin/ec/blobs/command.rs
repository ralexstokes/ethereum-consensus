use crate::blobs::{bundler, decode, encode};
use clap::{Args, Subcommand};
use std::io;

#[derive(Debug, Subcommand)]
enum Commands {
    Encode { framing: String },
    Decode { framing: String },
    Bundle,
}

#[derive(Debug, Args)]
#[clap(about = "utilities for blobspace")]
pub struct Command {
    #[clap(subcommand)]
    command: Commands,
}

impl Command {
    pub fn execute(self) -> eyre::Result<()> {
        match self.command {
            Commands::Encode { framing, .. } => {
                let stdin = io::stdin().lock();
                let blobs = encode::from_reader(stdin, framing.try_into()?)?;
                let result = serde_json::to_string_pretty(&blobs)?;
                println!("{}", result);
                Ok(())
            }
            Commands::Decode { framing, .. } => {
                let stdin = io::stdin().lock();
                let stdout = io::stdout().lock();
                decode::to_writer_from_json(stdin, stdout, framing.try_into()?)?;
                Ok(())
            }
            Commands::Bundle => {
                let stdin = io::stdin().lock();
                let blobs_bundle = bundler::from_reader(stdin)?;
                let result = serde_json::to_string_pretty(&blobs_bundle)?;
                println!("{}", result);
                Ok(())
            }
        }
    }
}
