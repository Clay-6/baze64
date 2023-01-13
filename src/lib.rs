use std::collections::HashMap;

use bitreader::BitReader;
use lazy_static::lazy_static;

/// A string of Base64 encoded data
#[derive(Debug, Clone)]
pub struct Base64String(String);

impl Base64String {
    pub const PADDING: char = '=';
    const ENCODE_MAP: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    /// Encodes a sequence of bytes into a [`Base64String`]
    pub fn encode(bytes: &[u8]) -> Self {
        let mut chunks = bytes.chunks_exact(3);
        let mut encoded = vec![];

        #[allow(clippy::while_let_on_iterator)] // Ownership shenanigans necessitate this
        while let Some(chunk) = chunks.next() {
            encoded.push(Self::encode_triplet(&[chunk[0], chunk[1], chunk[2]]))
        }

        let rem = chunks.remainder();
        match rem.len() {
            0 => { /* Do nothing */ }
            1 => encoded.push(Self::encode_singlet(rem)),
            2 => encoded.push(Self::encode_doublet(rem)),
            _ => unreachable!("{}", rem.len()), // Mathematically impossible
        }
        Self(encoded.iter().flatten().collect())
    }

    /// Returns a URL-safe version of a [`Base64String`]
    pub fn to_url_safe(&self) -> String {
        self.0.replace('+', "-").replace('/', "_")
    }

    /// Constructs a [`Base64String`] from a URL-safe
    /// Base64 encoded string
    pub fn from_url_safe(b64: &str) -> Self {
        Self(b64.replace('-', "+").replace('_', "/"))
    }

    /// Encodes a set of 3 bytes
    fn encode_triplet(triple: &[u8; 3]) -> [char; 4] {
        let mut reader = BitReader::new(triple);
        // These unwraps are fine because 8*3 == 6*4
        let first = reader.read_u8(6).unwrap();
        let second = reader.read_u8(6).unwrap();
        let third = reader.read_u8(6).unwrap();
        let fourth = reader.read_u8(6).unwrap();

        [
            Self::ENCODE_MAP[first as usize],
            Self::ENCODE_MAP[second as usize],
            Self::ENCODE_MAP[third as usize],
            Self::ENCODE_MAP[fourth as usize],
        ]
    }

    /// Encodes a single byte & pads it
    fn encode_singlet(rem: &[u8]) -> [char; 4] {
        let mut reader = BitReader::new(rem);
        let six = reader.read_u8(6).unwrap();
        let half_nib = reader.read_u8(2).unwrap();
        let (half_nib, _) = half_nib.overflowing_shl(4);

        // let padded = half_nib
        //     .replace_bits(4..5, half_nib.extract_bits(0..1))
        //     .replace_bits(0..3, 0);

        let first = Self::ENCODE_MAP[six as usize];
        let second = Self::ENCODE_MAP[half_nib as usize];

        [first, second, Self::PADDING, Self::PADDING]
    }

    /// Encodes a set of 2 bytes & pads it
    fn encode_doublet(rem: &[u8]) -> [char; 4] {
        let mut reader = BitReader::new(rem);
        let six1 = reader.read_u8(6).unwrap();
        let six2 = reader.read_u8(6).unwrap();
        let nibble = reader.read_u8(4).unwrap();
        let (nibble, _) = nibble.overflowing_shl(2);

        let first = Self::ENCODE_MAP[six1 as usize];
        let second = Self::ENCODE_MAP[six2 as usize];
        let third = Self::ENCODE_MAP[nibble as usize];

        [first, second, third, Self::PADDING]
    }
}

impl PartialEq for Base64String {
    fn eq(&self, other: &Self) -> bool {
        self.0.chars().filter(|&c| c != '=').collect::<String>()
            == other.0.chars().filter(|&c| c != '=').collect::<String>()
    }
}

impl core::fmt::Display for Base64String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

lazy_static! {
    static ref DECODE_MAP: HashMap<char, usize> = HashMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2),
        ('D', 3),
        ('E', 4),
        ('F', 5),
        ('G', 6),
        ('H', 7),
        ('I', 8),
        ('J', 9),
        ('K', 10),
        ('L', 11),
        ('M', 12),
        ('N', 13),
        ('O', 14),
        ('P', 15),
        ('Q', 16),
        ('R', 17),
        ('S', 18),
        ('T', 19),
        ('U', 20),
        ('V', 21),
        ('W', 22),
        ('X', 23),
        ('Y', 24),
        ('Z', 25),
        ('a', 26),
        ('b', 27),
        ('c', 28),
        ('d', 29),
        ('e', 30),
        ('f', 31),
        ('g', 32),
        ('h', 33),
        ('i', 34),
        ('j', 35),
        ('k', 36),
        ('l', 37),
        ('m', 38),
        ('n', 39),
        ('o', 40),
        ('p', 41),
        ('q', 42),
        ('r', 43),
        ('s', 44),
        ('t', 45),
        ('u', 46),
        ('v', 47),
        ('w', 48),
        ('x', 49),
        ('y', 50),
        ('z', 51),
        ('0', 52),
        ('1', 53),
        ('2', 54),
        ('3', 55),
        ('4', 56),
        ('5', 57),
        ('6', 58),
        ('7', 59),
        ('8', 60),
        ('9', 61),
        ('+', 62),
        ('/', 63),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn encode_triplet() {
        let triplet = ['A', 'B', 'C'];
        let expected_encoded = ['Q', 'U', 'J', 'D'];

        let encoded =
            Base64String::encode_triplet(&[triplet[0] as u8, triplet[1] as u8, triplet[2] as u8]);

        assert_eq!(encoded, expected_encoded);
    }

    #[test]
    fn encode_long() {
        let input = "everybody".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>());
        let expected = Base64String(String::from("ZXZlcnlib2R5"));

        assert_eq!(b64, expected)
    }

    #[test]
    fn encode_2_rem() {
        let input = "event".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>());
        let expected = Base64String(String::from("ZXZlbnQ="));

        assert_eq!(b64, expected)
    }

    #[test]
    fn encode_1_rem() {
        let input = "even".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>());
        let expected = Base64String(String::from("ZXZlbg=="));

        assert_eq!(b64, expected)
    }
}
