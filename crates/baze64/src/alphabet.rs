use crate::B64Error;

/// Trait for a base64 alphabet that can be used
/// to encode & decode a [`Base64String`](crate::Base64String)
pub trait Alphabet {
    /// The padding character used for the alphabet
    fn padding(&self) -> Option<char>;

    /// Returns the base64 character corresponding to a set of 6
    /// bits
    ///
    /// [`Base64String`](crate::Base64String) makes sure to never
    /// call this with an out-of-range value, but this method
    /// should still handle it gracefully in case that does happen
    fn encode_bits(&self, bits: u8) -> Result<char, B64Error>;

    /// Decodes a base64 character into it's decoded value
    /// The returned value must fit in a 6-bit number,
    /// as [`Base64String`](crate::Base64String) will assume
    /// that this happens
    fn decode_char(&self, c: char) -> Result<u8, B64Error>;
}

/// The standard base64 alphabet as defined in
/// RFC 4648
#[derive(Debug, Clone, Copy)]
pub struct Standard {
    encode_map: [char; 64],
}

impl Standard {
    pub const fn new() -> Self {
        Self {
            encode_map: [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
            ],
        }
    }
}

impl Default for Standard {
    /// Get a new [`Standard`] alphabet, using its [`Default`] impl
    fn default() -> Self {
        Self::new()
    }
}

impl UrlSafe {
    /// Get a new [`UrlSafe`] Alphabet, using its [`Default`] impl
    pub const fn new() -> Self {
        Self {
            encode_map: [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
            ],
        }
    }
}

impl Default for UrlSafe {
    fn default() -> Self {
        Self::new()
    }
}

/// The URL safe base64 alphabet as defined in
/// RFC 4648
#[derive(Debug, Clone, Copy)]
pub struct UrlSafe {
    encode_map: [char; 64],
}

impl Alphabet for Standard {
    fn padding(&self) -> Option<char> {
        Some('=')
    }

    fn encode_bits(&self, bits: u8) -> Result<char, B64Error> {
        if bits > 63 {
            Err(B64Error::BitsOOB(bits))
        } else {
            Ok(self.encode_map[bits as usize])
        }
    }

    fn decode_char(&self, c: char) -> Result<u8, B64Error> {
        if c == self.padding().unwrap() {
            Ok(0)
        } else if c == '\0' {
            Ok(0x64)
        } else {
            self.encode_map
                .iter()
                .position(|&ch| ch == c)
                .map_or_else(|| Err(B64Error::InvalidChar(c)), |i| Ok(i as u8))
        }
    }
}

impl Alphabet for UrlSafe {
    fn padding(&self) -> Option<char> {
        Some('=')
    }

    fn encode_bits(&self, bits: u8) -> Result<char, B64Error> {
        if bits > 63 {
            Err(B64Error::BitsOOB(bits))
        } else {
            Ok(self.encode_map[bits as usize])
        }
    }

    fn decode_char(&self, c: char) -> Result<u8, B64Error> {
        if c == self.padding().unwrap() {
            Ok(0)
        } else if c == '\0' {
            Ok(0x64)
        } else {
            self.encode_map
                .iter()
                .position(|&ch| ch == c)
                .map_or_else(|| Err(B64Error::InvalidChar(c)), |i| Ok(i as u8))
        }
    }
}
