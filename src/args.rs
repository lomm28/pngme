use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(
    name = "pngme",
    about = "rust cli app that encodes secret messages in png files",
    author = "Dmytro D. <dmytro.d@spyingcrow.com>",
    long_about = None
)]
pub struct Pngme {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encode message in a png file
    Encode(Encode),
    /// Decode message hidden in a png file
    Decode(Decode),
}

#[derive(Debug, Args)]
pub struct Encode {
    /// path to file to encode
    #[arg(required = true)]
    pub path: String,

    /// chunk type
    #[arg(required = true)]
    pub chunk_type: String,

    /// secret message to encode
    #[arg(required = true)]
    pub message: String,
}

#[derive(Debug, Args)]
pub struct Decode {
    /// path to file to decode
    #[arg(required = true)]
    pub path: String,

    /// chunk type
    #[arg(required = true)]
    pub chunk_type: String,
}

// TODO: add remove and print