mod generator;
mod type_generator;
mod visitors;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    command: Command,
}

#[derive(ValueEnum, Debug, Clone)]
enum Command {
    Forks,
    Types,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Forks => {
            generator::run();
        }
        Command::Types => {
            type_generator::run();
        }
    }
}
