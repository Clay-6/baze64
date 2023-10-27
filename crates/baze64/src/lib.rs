//! Encode & decode base64 to & from arbitrary byte sequences
//!
//! All encoding & decoding is done via the [`Base64String`]
//! struct, using an alphabet implementing the [`Alphabet`](alphabet::Alphabet)
//! trait
//!
//! ## Examples
//!
//! Encode & decode a string:
//! ```
//! # use baze64::{Base64String, alphabet::Standard};
//! let text = "Some text".to_string();
//! let encoded = Base64String::<Standard>::encode(text.as_bytes());
//! let decoded = encoded.decode()?;
//!
//! assert_eq!(text, String::from_utf8(decoded)?);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Encode & decode a file:
//! ```no_run
//! # use std::{fs::File, io::Read};
//! # use baze64::{Base64String, alphabet::Standard};
//! let mut file = File::open("path/to/file.ext")?;
//! let mut buffer = Vec::new();
//! file.read_to_end(&mut buffer)?;
//! let encoded = Base64String::<Standard>::encode(&buffer);
//! let bytes = encoded.decode()?;
//!
//! assert_eq!(buffer, bytes);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!

pub mod alphabet;
mod base64string;

pub use base64string::Base64String;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum B64Error {
    #[error("Value `{0}` is outsite the 6-bit integer range")]
    BitsOOB(u8),
    #[error("Invalid Base64 character `{0}`")]
    InvalidChar(char),
}
