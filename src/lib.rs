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
//! let encoded: Base64String<Standard> = Base64String::encode(text.as_bytes());
//! let decoded = encoded.decode();
//!
//! assert_eq!(text, String::from_utf8(decoded)?);
//! # Ok::<(), std::string::FromUtf8Error>(())
//! ```
//!
//! Encode & decode a file:
//! ```no_run
//! # use std::{fs::File, io::Read};
//! # use baze64::{Base64String, alphabet::Standard};
//! let mut file = File::open("path/to/file.ext")?;
//! let mut buffer = Vec::new();
//! file.read_to_end(&mut buffer)?;
//! let encoded: Base64String<Standard> = Base64String::encode(&buffer);
//! let bytes = encoded.decode();
//!
//! assert_eq!(buffer, bytes);
//! # Ok::<(), std::io::Error>(())
//! ```
//!

pub mod alphabet;
mod base64string;

pub use base64string::Base64String;
