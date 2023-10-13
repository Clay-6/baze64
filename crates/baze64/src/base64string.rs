use std::io::Write;

use crate::{alphabet::Alphabet, B64Error};

/// A string of Base64 encoded data
#[derive(Debug, Clone)]
pub struct Base64String<A> {
    content: String,
    alphabet: A,
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error(transparent)]
    Base64Error(#[from] B64Error),
    #[error(transparent)]
    WriteError(#[from] std::io::Error),
    #[error(transparent)]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

impl<A> Base64String<A>
where
    A: Alphabet,
{
    /// Encode a sequence of bytes into a [`Base64String`] using a
    /// given `alphabet` instance
    ///
    /// # Examples
    /// ```
    /// # use baze64::*;
    /// # type MyAlphabet = baze64::alphabet::Standard;
    ///
    /// let data = "secret message".as_bytes();
    /// let alphabet = MyAlphabet::new();
    /// let encoded = Base64String::encode_with(&data, alphabet)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn encode_with<B>(bytes: B, alphabet: A) -> Result<Self, B64Error>
    where
        B: AsRef<[u8]>,
    {
        let bytes = bytes.as_ref();
        let padding = alphabet.padding().unwrap_or_default();

        let chunks = bytes.chunks(3);
        let mut encoded = vec![];

        for chunk in chunks {
            match chunk.len() {
                3 => encoded.push(Self::encode_triplet(
                    [chunk[0], chunk[1], chunk[2]],
                    &alphabet,
                )?),
                2 => {
                    let res = Self::encode_triplet([chunk[0], chunk[1], 0x00], &alphabet)?;
                    encoded.push([res[0], res[1], res[2], padding])
                }
                1 => {
                    let res = Self::encode_triplet([chunk[0], 0x00, 0x00], &alphabet)?;
                    encoded.push([res[0], res[1], padding, padding])
                }
                _ => unreachable!("Mathematically impossible"),
            }
        }

        Ok(Self {
            content: encoded.iter().flatten().collect(),
            alphabet,
        })
    }

    /// Decode the contents of `self` into a byte sequence
    ///
    /// # Examples
    /// ```
    /// # use baze64::{Base64String, alphabet::Standard};
    /// let data = "Pretend this is important";
    /// let base64 = Base64String::<Standard>::encode(data.as_bytes())?;
    /// let decoded_bytes = base64.decode()?;
    ///
    /// assert_eq!(data.as_bytes(), &decoded_bytes);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn decode(&self) -> Result<Vec<u8>, DecodeError> {
        let mut decoded = vec![];

        self.decode_into(&mut decoded)?;

        Ok(decoded)
    }

    /// Decode the contents of `self` into the `buf` provided
    ///
    /// # Examples
    /// ```no_run
    /// # use baze64::{Base64String, alphabet::Standard};
    /// # use std::{fs::File, io::Read};
    ///
    /// let data = "Definitely not contrived";
    /// let base64 = Base64String::<Standard>::encode(data.as_bytes())?;
    /// let mut file = File::open("some/file.txt")?;
    /// base64.decode_into(&mut file)?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn decode_into<O>(&self, buf: &mut O) -> Result<(), DecodeError>
    where
        O: Write,
    {
        let padding = self.alphabet.padding().unwrap_or_default();
        let tmp = self.content.chars().collect::<Vec<_>>();
        let segments = tmp.chunks_exact(4);

        for seg in segments {
            if seg.ends_with(&[padding, padding]) || seg.len() % 4 == 2 {
                let tri =
                    Self::decode_quad([seg[0], seg[1], 0 as char, 0 as char], &self.alphabet)?;
                buf.write_all(&[tri[0]])?;
            } else if seg.ends_with(&[padding]) || seg.len() % 4 == 3 {
                let tri = Self::decode_quad([seg[0], seg[1], seg[2], 0 as char], &self.alphabet)?;
                buf.write_all(&tri[0..2])?;
            } else {
                let tri = Self::decode_quad([seg[0], seg[1], seg[2], seg[3]], &self.alphabet)?;
                buf.write_all(&tri)?;
            }
        }

        Ok(())
    }

    /// Decode the contents of `self` into a [`String`]
    ///
    /// # Examples
    /// ```
    /// # use baze64::{Base64String, alphabet::Standard};
    /// let message = "Secret message :D";
    /// let encoded = Base64String::<Standard>::encode(message.as_bytes())?;
    /// let decoded = encoded.decode_to_string()?;
    ///
    /// assert_eq!(message, decoded.as_str());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn decode_to_string(&self) -> Result<String, DecodeError> {
        let string = String::from_utf8(self.decode()?)?;
        Ok(string)
    }

    /// Contruct a [`Base64String`] from already encoded
    /// Base64
    ///
    /// # Examples
    /// ```
    /// # use baze64::{Base64String, alphabet::Standard};
    /// // Pretend this does something useful
    /// fn do_something(something_encoded: &str) {
    ///     let base64 = Base64String::from_encoded_with(something_encoded, Standard::new());
    ///     // Now to use the Base64String!
    /// }
    /// ```
    pub fn from_encoded_with<S>(b64: S, alphabet: A) -> Self
    where
        S: ToString,
    {
        let mut content = b64.to_string();
        if let Some(p) = alphabet.padding() {
            while content.len() % 4 != 0 {
                content.push(p)
            }
        }

        Self { content, alphabet }
    }

    /// Returns the encoded string with the padding removed
    ///
    /// # Example
    /// ```
    /// # use baze64::{Base64String, alphabet::Standard};
    /// let padded = Base64String::<Standard>::encode("Something important".as_bytes())?;
    /// let unpadded = padded.without_padding();
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn without_padding(&self) -> String {
        self.content
            .chars()
            .filter(|&c| c != self.alphabet.padding().unwrap_or_default())
            .collect()
    }

    /// Change a [`Base64String`] to the specified
    /// alphabet `B` using the given `target_alphabet` instance of `B`
    ///
    /// # Examples
    /// ```
    /// # use baze64::{Base64String, alphabet::{Standard, UrlSafe}};
    /// let data = "Something important".as_bytes();
    /// let standard = Base64String::<Standard>::encode(&data)?;
    /// let url_safe = standard.change_alphabet_with(UrlSafe::new())?;
    ///
    /// assert_eq!(data, url_safe.decode()?);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn change_alphabet_with<B>(self, target_alphabet: B) -> Result<Base64String<B>, DecodeError>
    where
        B: Alphabet,
    {
        let inner = self.decode()?;

        Ok(Base64String::encode_with(inner, target_alphabet)?)
    }

    /// Decode a set of 4 bytes
    ///
    /// Bit fuckery courtesey of
    /// [Matheus Gomes](https://matgomes.com/base64-encode-decode-cpp)
    fn decode_quad([a, b, c, d]: [char; 4], alphabet: &A) -> Result<[u8; 3], B64Error> {
        let concat_bytes = ((alphabet.decode_char(a)? as u32) << 18)
            | ((alphabet.decode_char(b)? as u32) << 12)
            | ((alphabet.decode_char(c)? as u32) << 6)
            | alphabet.decode_char(d)? as u32;
        Ok([
            ((concat_bytes >> 16) & 0b1111_1111) as u8,
            ((concat_bytes >> 8) & 0b1111_1111) as u8,
            (concat_bytes & 0b1111_1111) as u8,
        ])
    }

    /// Encodes a set of 3 bytes
    fn encode_triplet([a, b, c]: [u8; 3], alphabet: &A) -> Result<[char; 4], B64Error> {
        let concated = ((a as u32) << 16) | ((b as u32) << 8) | c as u32;
        // These unwraps are fine because 8*3 == 6*4
        let first = ((concated >> 18) & 0b0011_1111) as u8;
        let second = ((concated >> 12) & 0b0011_1111) as u8;
        let third = ((concated >> 6) & 0b0011_1111) as u8;
        let fourth = (concated & 0b0011_1111) as u8;

        Ok([
            alphabet.encode_bits(first)?,
            alphabet.encode_bits(second)?,
            alphabet.encode_bits(third)?,
            alphabet.encode_bits(fourth)?,
        ])
    }
}

impl<A> Base64String<A>
where
    A: Alphabet + Default,
{
    /// Encode a sequence of bytes into a [`Base64String`]
    ///
    /// Uses `A`'s [`Default`] impl as the alphabet
    /// to encode with
    ///
    /// # Examples
    /// ```
    /// # use baze64::{Base64String, alphabet::Standard};
    ///
    /// let data = "secret message".as_bytes();
    /// let encoded = Base64String::<Standard>::encode(data)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn encode<B>(bytes: B) -> Result<Self, B64Error>
    where
        B: AsRef<[u8]>,
    {
        Self::encode_with(bytes, A::default())
    }

    /// Contruct a [`Base64String`] from already encoded
    /// Base64
    ///
    /// Uses `A`'s [`Default`] impl as the alphabet to encode
    /// with
    ///
    /// # Examples
    /// ```
    /// # use baze64::{Base64String, alphabet::Standard};
    /// // Pretend this does something useful
    /// fn do_something(something_encoded: &str) {
    ///     let base64 = Base64String::<Standard>::from_encoded(something_encoded);
    ///     // Now to use the Base64String!
    /// }
    /// ```
    pub fn from_encoded<S>(b64: S) -> Self
    where
        S: ToString,
    {
        Self::from_encoded_with(b64, A::default())
    }
}

impl<A> core::fmt::Display for Base64String<A>
where
    A: Alphabet,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<A> PartialEq for Base64String<A>
where
    A: Alphabet,
{
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl<A> AsRef<str> for Base64String<A>
where
    A: Alphabet,
{
    fn as_ref(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use crate::alphabet::Standard;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn encode_triplet() {
        let triplet = ['A', 'B', 'C'];
        let expected_encoded = ['Q', 'U', 'J', 'D'];

        let encoded = Base64String::<Standard>::encode_triplet(
            [triplet[0] as u8, triplet[1] as u8, triplet[2] as u8],
            &Standard::new(),
        )
        .unwrap();

        assert_eq!(encoded, expected_encoded);
    }

    #[test]
    fn encode_long() {
        let input = "everybody".chars().map(|c| c as u8);
        let b64 = Base64String::encode(input.collect::<Vec<_>>()).unwrap();
        let expected = Base64String {
            content: String::from("ZXZlcnlib2R5"),
            alphabet: Standard::new(),
        };

        assert_eq!(b64, expected)
    }

    #[test]
    fn encode_2_rem() {
        let input = "event".chars().map(|c| c as u8);
        let b64 = Base64String::encode(input.collect::<Vec<_>>()).unwrap();
        let expected = Base64String {
            content: String::from("ZXZlbnQ="),
            alphabet: Standard::new(),
        };

        assert_eq!(b64, expected)
    }

    #[test]
    fn encode_1_rem() {
        let input = "even".chars().map(|c| c as u8);
        let b64 = Base64String::encode(input.collect::<Vec<_>>()).unwrap();
        let expected = Base64String {
            content: String::from("ZXZlbg=="),
            alphabet: Standard::new(),
        };

        assert_eq!(b64, expected)
    }

    #[test]
    fn decode_no_pad() {
        let src = Base64String {
            content: String::from("ZXZlcnlib2R5"),
            alphabet: Standard::new(),
        };
        let expected = b"everybody".to_vec();
        let decoded = src.decode().unwrap();

        assert_eq!(decoded, expected)
    }

    #[test]
    fn decode_one_pad() {
        let src = Base64String {
            content: String::from("ZXZlbnQ="),
            alphabet: Standard::new(),
        };
        let expected = b"event".to_vec();
        let decoded = src.decode().unwrap();

        assert_eq!(decoded, expected)
    }

    #[test]
    fn decode_two_pad() {
        let src = Base64String {
            content: String::from("ZXZlbg=="),
            alphabet: Standard::new(),
        };
        let expected = b"even".to_vec();
        let decoded = src.decode().unwrap();

        assert_eq!(decoded, expected)
    }
}
