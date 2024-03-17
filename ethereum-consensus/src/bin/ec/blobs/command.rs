use crate::blobs::{bundler, decode, encode, framing::Mode as Framing};
use clap::{Args, Subcommand, ValueEnum};
use std::io;

#[derive(Debug, Subcommand)]
enum Commands {
    Encode {
        #[arg(value_enum, default_value_t)]
        framing: FramingArg,
    },
    Decode {
        #[arg(value_enum, default_value_t)]
        framing: FramingArg,
    },
    Bundle,
}

#[derive(Debug, Clone, Default, ValueEnum)]
enum FramingArg {
    Raw,
    #[default]
    Sized,
}

impl From<FramingArg> for Framing {
    fn from(value: FramingArg) -> Self {
        match value {
            FramingArg::Raw => Framing::Raw,
            FramingArg::Sized => Framing::Sized,
        }
    }
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
            Commands::Encode { framing } => {
                let stdin = io::stdin().lock();
                let blobs = encode::from_reader(stdin, framing.into())?;
                let result = serde_json::to_string_pretty(&blobs)?;
                println!("{}", result);
                Ok(())
            }
            Commands::Decode { framing } => {
                let stdin = io::stdin().lock();
                let stdout = io::stdout().lock();
                decode::to_writer_from_json(stdin, stdout, framing.into())?;
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
