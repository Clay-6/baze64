use bitreader::BitReader;
use simple_bits::BitsExt;

/// A string of Base64 encoded data
#[derive(Debug, Clone)]
pub struct Base64String(String);

impl Base64String {
    pub const PADDING: char = '=';
    pub const ENCODE_MAP: [char; 64] = [
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

        let padded = half_nib
            .replace_bits(4..5, half_nib.extract_bits(0..1))
            .replace_bits(0..3, 0);

        let first = Self::ENCODE_MAP[six as usize];
        let second = Self::ENCODE_MAP[padded as usize];

        [first, second, Self::PADDING, Self::PADDING]
    }

    /// Encodes a set of 2 bytes & pads it
    fn encode_doublet(rem: &[u8]) -> [char; 4] {
        let mut reader = BitReader::new(rem);
        let six1 = reader.read_u8(6).unwrap();
        let six2 = reader.read_u8(6).unwrap();
        let nibble = reader.read_u8(4).unwrap();
        let padded = nibble
            .replace_bits(2..6, nibble.extract_bits(0..4))
            .replace_bits(0..2, 0)
            .replace_bits(7..8, 0);

        let first = Self::ENCODE_MAP[six1 as usize];
        let second = Self::ENCODE_MAP[six2 as usize];
        let third = Self::ENCODE_MAP[padded as usize];

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
