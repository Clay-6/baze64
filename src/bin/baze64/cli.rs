use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Encode something into Base64
    Encode {
        /// Encode UTF-8 text
        text: Option<String>,
        /// Encode a file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    /// Decode a Base64 string
    Decode {
        /// The Base64 string to decode
        text: String,
        /// The output file for the decoded data
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
}
