use std::marker::PhantomData;

use crate::{alphabet::Alphabet, B64Error};

/// A string of Base64 encoded data
#[derive(Debug, Clone)]
pub struct Base64String<A> {
    content: String,
    _alphabet: PhantomData<A>,
}

impl<A> Base64String<A>
where
    A: Alphabet,
{
    /// Encode a sequence of bytes into a [`Base64String`]
    pub fn encode(bytes: &[u8]) -> Result<Self, B64Error> {
        let padding = A::PADDING.unwrap_or_default();

        let chunks = bytes.chunks(3);
        let mut encoded = vec![];

        for chunk in chunks {
            match chunk.len() {
                3 => encoded.push(Self::encode_triplet([chunk[0], chunk[1], chunk[2]])?),
                2 => {
                    let res = Self::encode_triplet([chunk[0], chunk[1], 0x00])?;
                    encoded.push([res[0], res[1], res[2], padding])
                }
                1 => {
                    let res = Self::encode_triplet([chunk[0], 0x00, 0x00])?;
                    encoded.push([res[0], res[1], padding, padding])
                }
                _ => unreachable!("Mathematically impossible"),
            }
        }

        Ok(Self {
            content: encoded.iter().flatten().collect(),
            _alphabet: PhantomData,
        })
    }

    /// Decode the contents of `self` into a byte sequence
    pub fn decode(&self) -> Result<Vec<u8>, B64Error> {
        let padding = A::PADDING.unwrap_or_default();
        let mut decoded = vec![];
        let tmp = self.content.chars().collect::<Vec<_>>();
        let segments = tmp.chunks_exact(4);

        for seg in segments {
            if seg.ends_with(&[padding, padding]) || seg.len() % 4 == 2 {
                let tri = Self::decode_quad([seg[0], seg[1], 0 as char, 0 as char])?;
                decoded.push(tri[0]);
            } else if seg.ends_with(&[padding]) || seg.len() % 4 == 3 {
                let tri = Self::decode_quad([seg[0], seg[1], seg[2], 0 as char])?;
                decoded.extend_from_slice(&tri[0..2])
            } else {
                let tri = Self::decode_quad([seg[0], seg[1], seg[2], seg[3]])?;
                decoded.extend_from_slice(&tri)
            }
        }

        Ok(decoded)
    }

    /// Contruct a [`Base64String`] from already encoded
    /// Base64
    pub fn from_encoded(b64: &str) -> Self {
        let mut content = b64.to_string();
        if let Some(p) = A::PADDING {
            while content.len() % 4 != 0 {
                content.push(p)
            }
        }

        Self {
            content,
            _alphabet: PhantomData::<A>,
        }
    }

    /// Returns the encoded string with the padding removed
    pub fn without_padding(&self) -> String {
        self.content
            .chars()
            .filter(|&c| c != A::PADDING.unwrap_or_default())
            .collect()
    }

    /// Returns a new [`Base64String`] with the specified
    /// alphabet `B`
    pub fn change_alphabet<B>(self) -> Result<Base64String<B>, B64Error>
    where
        B: Alphabet,
    {
        let inner = self.decode()?;

        Base64String::<B>::encode(&inner)
    }

    /// Decode a set of 4 bytes
    ///
    /// Bit fuckery courtesey of
    /// [Matheus Gomes](https://matgomes.com/base64-encode-decode-cpp)
    fn decode_quad([a, b, c, d]: [char; 4]) -> Result<[u8; 3], B64Error> {
        let concat_bytes = ((A::decode_char(a)? as u32) << 18)
            | ((A::decode_char(b)? as u32) << 12)
            | ((A::decode_char(c)? as u32) << 6)
            | A::decode_char(d)? as u32;
        Ok([
            ((concat_bytes >> 16) & 0b1111_1111) as u8,
            ((concat_bytes >> 8) & 0b1111_1111) as u8,
            (concat_bytes & 0b1111_1111) as u8,
        ])
    }

    /// Encodes a set of 3 bytes
    fn encode_triplet([a, b, c]: [u8; 3]) -> Result<[char; 4], B64Error> {
        let concated = ((a as u32) << 16) | ((b as u32) << 8) | c as u32;
        // These unwraps are fine because 8*3 == 6*4
        let first = ((concated >> 18) & 0b0011_1111) as u8;
        let second = ((concated >> 12) & 0b0011_1111) as u8;
        let third = ((concated >> 6) & 0b0011_1111) as u8;
        let fourth = (concated & 0b0011_1111) as u8;

        Ok([
            A::encode_bits(first)?,
            A::encode_bits(second)?,
            A::encode_bits(third)?,
            A::encode_bits(fourth)?,
        ])
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

#[cfg(test)]
mod tests {
    use crate::alphabet::Standard;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn encode_triplet() {
        let triplet = ['A', 'B', 'C'];
        let expected_encoded = ['Q', 'U', 'J', 'D'];

        let encoded = Base64String::<Standard>::encode_triplet([
            triplet[0] as u8,
            triplet[1] as u8,
            triplet[2] as u8,
        ])
        .unwrap();

        assert_eq!(encoded, expected_encoded);
    }

    #[test]
    fn encode_long() {
        let input = "everybody".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>()).unwrap();
        let expected = Base64String {
            content: String::from("ZXZlcnlib2R5"),
            _alphabet: PhantomData::<Standard>,
        };

        assert_eq!(b64, expected)
    }

    #[test]
    fn encode_2_rem() {
        let input = "event".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>()).unwrap();
        let expected = Base64String {
            content: String::from("ZXZlbnQ="),
            _alphabet: PhantomData::<Standard>,
        };

        assert_eq!(b64, expected)
    }

    #[test]
    fn encode_1_rem() {
        let input = "even".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>()).unwrap();
        let expected = Base64String {
            content: String::from("ZXZlbg=="),
            _alphabet: PhantomData::<Standard>,
        };

        assert_eq!(b64, expected)
    }

    #[test]
    fn decode_no_pad() {
        let src = Base64String {
            content: String::from("ZXZlcnlib2R5"),
            _alphabet: PhantomData::<Standard>,
        };
        let expected = b"everybody".to_vec();
        let decoded = src.decode().unwrap();

        assert_eq!(decoded, expected)
    }

    #[test]
    fn decode_one_pad() {
        let src = Base64String {
            content: String::from("ZXZlbnQ="),
            _alphabet: PhantomData::<Standard>,
        };
        let expected = b"event".to_vec();
        let decoded = src.decode().unwrap();

        assert_eq!(decoded, expected)
    }

    #[test]
    fn decode_two_pad() {
        let src = Base64String {
            content: String::from("ZXZlbg=="),
            _alphabet: PhantomData::<Standard>,
        };
        let expected = b"even".to_vec();
        let decoded = src.decode().unwrap();

        assert_eq!(decoded, expected)
    }
}
