//! Various byte or byte collection manipulations.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HexParseError {
    #[error("Invalid characters in input")]
    ParseError,
    #[error("Input's length must be a multiple of 2")]
    InputError,
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

#[derive(Error, Debug)]
pub enum FixedXorError {
    #[error("Sources differ in their lengths")]
    SourcesDifferingLength,
}

/// Take two sequences of bytes of the same length and XOR them together.
pub fn fixed_xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, FixedXorError> {
    if a.len() != b.len() {
        Err(FixedXorError::SourcesDifferingLength)
    } else {
        Ok(a.iter().zip(b.iter()).map(|(l, r)| *l ^ *r).collect())
    }
}

// Base64 stuff

const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

/// Encode a stream of bytes into a Base64 `String`.
///
/// # Example
///
/// ```
/// let ex = [255u8, 0, 0x11, 0x22, 0x44, 0x88, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0];
/// let result = encode(&ex);
/// assert_eq!("/wARIkSIqrvM3e7/AA==", result);
/// ```
pub fn encode(bytes: &[u8]) -> String {
    let encoded_bytes = encode_bytes(bytes);
    let mut result = encoded_bytes
        .iter()
        .map(|&b| BASE64[b as usize])
        .collect::<String>();

    // Pad the output to a multiple of 4
    while result.len() % 4 != 0 {
        result.push('=');
    }

    result
}

pub fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity((bytes.len() * 4) / 3);
    let mut index: usize = 0;

    // Whizz through the easy bits: if you have a clear three bytes available
    // they become four Base64 characters.

    let mut intermediate: u32;
    while index + 2 < bytes.len() {
        intermediate = (0..=2).fold(0u32, |acc, i| acc << 8 | bytes[index + i] as u32);
        result.push(((intermediate >> 18) & 63) as u8);
        result.push(((intermediate >> 12) & 63) as u8);
        result.push(((intermediate >> 6) & 63) as u8);
        result.push(((intermediate) & 63) as u8);
        index += 3;
    }

    // By this point you'll have zero, one or two bytes left.

    // If there's a first byte it'll have its six bits in there verbatim.
    if bytes.len() % 3 > 0 {
        result.push(((bytes[index] >> 2) & 63) as u8);

        // The remaining two bits have to be included in whatever follows and if
        // there's a next byte we take the high four bits of that too.
        let mut intermediate = (bytes[index] << 4) & 48;
        if bytes.len() % 3 > 1 {
            intermediate |= (bytes[index + 1] >> 4) & 15;
        }
        result.push(intermediate);

        // If there _was_ a next byte, there's only four bits to take account of.
        if bytes.len() % 3 > 1 {
            result.push((bytes[index] << 2) & 60);
        }
    }

    result
}

//
// TESTS
//

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

    #[test]
    fn test_fixed_xor() {
        let tests = [
            ("", "", ""),
            (
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965",
                "746865206b696420646f6e277420706c6179",
            ),
        ];
        for &(a, b, expected) in tests.iter() {
            let a = hex_to_bytes(a).unwrap();
            let b = hex_to_bytes(b).unwrap();
            let expected = hex_to_bytes(expected).unwrap();
            let actual = fixed_xor(&a, &b).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_encode() {
        let tests = [
            (vec![], ""),
            (vec![0u8], "AA=="),
            (vec![0u8, 0], "AAA="),
            (vec![0u8, 0, 0], "AAAA"),
            (vec![255u8], "/w=="),
            (vec![255u8, 255], "//8="),
            (vec![255u8, 255, 255], "////"),
            (
                vec![
                    255u8, 0, 0x11, 0x22, 0x44, 0x88, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0,
                ],
                "/wARIkSIqrvM3e7/AA==",
            ),
            (
                vec![
                    0x49u8, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x79,
                    0x6f, 0x75, 0x72, 0x20, 0x62, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b,
                    0x65, 0x20, 0x61, 0x20, 0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73,
                    0x20, 0x6d, 0x75, 0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d,
                ],
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            ),
        ];
        for (source, expected) in tests.iter() {
            let actual = encode(source);
            assert_eq!(expected, &actual);
        }
    }
}
