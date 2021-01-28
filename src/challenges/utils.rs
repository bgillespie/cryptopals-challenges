///! Some random stuff
use std::num::ParseIntError;

#[derive(Debug)]
pub enum HexParseError {
    ParseError,
    InputError,
}

impl From<ParseIntError> for HexParseError {
    fn from(_: ParseIntError) -> Self {
        HexParseError::ParseError
    }
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, HexParseError> {
    if hex.len() % 2 != 0 {
        Err(HexParseError::InputError)
    } else {
        (0..hex.len())
            .step_by(2)
            .map(|d| u8::from_str_radix(&hex[d..=d + 1], 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| HexParseError::ParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        // First: tests not resulting in errors
        let tests = [
            ("", vec![]),
            ("00", vec![0u8]),
            ("FF", vec![255u8]),
            ("ff", vec![255u8]),
            (
                "000102030405060708090a0b0c0d0e0f10",
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            ),
        ];

        for (sample, expected) in tests.iter() {
            let actual = hex_to_bytes(sample);
            assert_eq!(expected, &actual.unwrap());
        }
    }
}
