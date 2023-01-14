use std::marker::PhantomData;

use bitreader::BitReader;

use crate::alphabet::Alphabet;

/// A string of Base64 encoded data
#[derive(Debug, Clone)]
pub struct Base64String<A> {
    content: String,
    _marker: PhantomData<A>,
}

impl<A> Base64String<A>
where
    A: Alphabet,
{
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
        Self {
            content: encoded.iter().flatten().collect(),
            _marker: PhantomData,
        }
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
            A::ENCODE_MAP[first as usize],
            A::ENCODE_MAP[second as usize],
            A::ENCODE_MAP[third as usize],
            A::ENCODE_MAP[fourth as usize],
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

        let first = A::ENCODE_MAP[six as usize];
        let second = A::ENCODE_MAP[half_nib as usize];

        [first, second, A::PADDING, A::PADDING]
    }

    /// Encodes a set of 2 bytes & pads it
    fn encode_doublet(rem: &[u8]) -> [char; 4] {
        let mut reader = BitReader::new(rem);
        let six1 = reader.read_u8(6).unwrap();
        let six2 = reader.read_u8(6).unwrap();
        let nibble = reader.read_u8(4).unwrap();
        let (nibble, _) = nibble.overflowing_shl(2);

        let first = A::ENCODE_MAP[six1 as usize];
        let second = A::ENCODE_MAP[six2 as usize];
        let third = A::ENCODE_MAP[nibble as usize];

        [first, second, third, A::PADDING]
    }
}

impl<A> PartialEq for Base64String<A> {
    fn eq(&self, other: &Self) -> bool {
        self.content
            .chars()
            .filter(|&c| c != '=')
            .collect::<String>()
            == other
                .content
                .chars()
                .filter(|&c| c != '=')
                .collect::<String>()
    }
}

impl<A> core::fmt::Display for Base64String<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
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

        let encoded = Base64String::<Standard>::encode_triplet(&[
            triplet[0] as u8,
            triplet[1] as u8,
            triplet[2] as u8,
        ]);

        assert_eq!(encoded, expected_encoded);
    }

    #[test]
    fn encode_long() {
        let input = "everybody".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>());
        let expected = Base64String {
            content: String::from("ZXZlcnlib2R5"),
            _marker: PhantomData::<Standard>,
        };

        assert_eq!(b64, expected)
    }

    #[test]
    fn encode_2_rem() {
        let input = "event".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>());
        let expected = Base64String {
            content: String::from("ZXZlbnQ="),
            _marker: PhantomData::<Standard>,
        };

        assert_eq!(b64, expected)
    }

    #[test]
    fn encode_1_rem() {
        let input = "even".chars().map(|c| c as u8);
        let b64 = Base64String::encode(&input.collect::<Vec<_>>());
        let expected = Base64String {
            content: String::from("ZXZlbg=="),
            _marker: PhantomData::<Standard>,
        };

        assert_eq!(b64, expected)
    }
}
