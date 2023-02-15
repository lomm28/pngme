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
    Decode(DecodeRemove),
    /// Remove a chunk from png file
    Remove(DecodeRemove),
    /// Print chunks of the png file
    Print(Print),
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
pub struct DecodeRemove {
    /// path to file
    #[arg(required = true)]
    pub path: String,

    /// chunk type
    #[arg(required = true)]
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct Print {
    /// path to file to remove
    #[arg(required = true)]
    pub path: String,
}