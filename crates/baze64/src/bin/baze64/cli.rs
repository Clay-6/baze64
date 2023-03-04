use core::fmt;
use std::{path::PathBuf, str::FromStr};

use baze64::alphabet::{Standard, UrlSafe};
use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Report};

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
        /// Encode a UTF-8 string
        string: Option<String>,
        /// Encode a file
        #[clap(short, long)]
        file: Option<PathBuf>,
        /// The base64 alphabet to encode using
        #[clap(short, long, default_value_t = Alphabet::Standard)]
        alphabet: Alphabet,
        /// Return the encoded base64 without padding
        #[clap(long)]
        no_padding: bool,
    },
    /// Decode a Base64 string
    Decode {
        /// The Base64 string to decode
        base64: String,
        /// The output file for the decoded data
        #[clap(short, long)]
        output: Option<PathBuf>,
        /// The base64 alphabet the input was encoded in
        #[clap(short, long, default_value_t = Alphabet::Standard)]
        alphabet: Alphabet,
        /// Output the decoded data in hexadecimal form
        #[clap(short = 'H', long)]
        hex: bool,
        /// Output the decoded data in byte form
        #[clap(short, long)]
        bytes: bool,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Alphabet {
    Standard,
    UrlSafe,
}

impl baze64::alphabet::Alphabet for Alphabet {
    fn encode_bits(&self, bits: u8) -> Result<char, baze64::B64Error> {
        match self {
            Alphabet::Standard => Standard::new().encode_bits(bits),
            Alphabet::UrlSafe => UrlSafe::new().encode_bits(bits),
        }
    }

    fn decode_char(&self, c: char) -> Result<u8, baze64::B64Error> {
        match self {
            Alphabet::Standard => Standard::new().decode_char(c),
            Alphabet::UrlSafe => UrlSafe::new().decode_char(c),
        }
    }

    fn padding(&self) -> Option<char> {
        match self {
            Alphabet::Standard => Standard::new().padding(),
            Alphabet::UrlSafe => UrlSafe::new().padding(),
        }
    }
}

impl FromStr for Alphabet {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Self::Standard),
            "urlsafe" => Ok(Self::UrlSafe),
            _ => Err(eyre!(
                "Invalid alphabet specifier, use either `standard` or `urlsafe`"
            )),
        }
    }
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Alphabet::Standard => write!(f, "standard"),
            Alphabet::UrlSafe => write!(f, "urlsafe"),
        }
    }
}
