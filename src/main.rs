mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;
use args::{Pngme, Commands};
use commands::{encode, decode};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// cargo install --path . && pngme

fn main() -> Result<()> {
    let args = Pngme::parse();

    match args.command {
        Commands::Encode(name) => {
            encode(name)
        },
        Commands::Decode(name) => {
            decode(name)
        }
    }
}
