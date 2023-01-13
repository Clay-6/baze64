use bitreader::BitReader;

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

    pub fn encode(bytes: &[u8]) -> Self {
        todo!()
    }

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
}
