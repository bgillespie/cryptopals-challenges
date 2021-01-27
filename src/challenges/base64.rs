//! # base64
//!
//! `base64` just provides the `encode` function currently, per the first
//! Cryptopals set, challenge 1.

pub const BASE64: [char; 64] = [
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
    let mut result = String::new();
    let mut index: usize = 0;

    // Whizz through the easy bits: if you have a clear three bytes available
    // they become four Base64 characters.

    let mut intermediate: u32;
    while index + 2 < bytes.len() {
        intermediate = (0..=2).fold(0u32, |acc, i| acc << 8 | bytes[index + i] as u32);
        result.push(BASE64[((intermediate >> 18) & 63) as usize]);
        result.push(BASE64[((intermediate >> 12) & 63) as usize]);
        result.push(BASE64[((intermediate >> 6) & 63) as usize]);
        result.push(BASE64[((intermediate) & 63) as usize]);
        index += 3;
    }

    // By this point you'll have zero, one or two bytes left.

    // If there's a first byte it'll have its six bits in there verbatim.
    if bytes.len() % 3 > 0 {
        result.push(BASE64[((bytes[index] >> 2) & 63) as usize]);

        // The remaining two bits have to be included in whatever follows
        intermediate = ((bytes[index] << 4) & 48) as u32;
        // If there's a next byte we take the high bits of that too
        if bytes.len() % 3 > 1 {
            intermediate |= ((bytes[index + 1] >> 4) & 15) as u32;
        }
        result.push(BASE64[intermediate as usize]);

        // Now if there _was_ a next byte...
        if bytes.len() % 3 > 1 {
            // ... add the remaining bits as the higher bits of the final number
            result.push(BASE64[((bytes[index] << 2) & 60) as usize]);
        }
    }

    // Pad the output to a multiple of 4
    while result.len() % 4 != 0 {
        result.push('=');
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

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
                vec![255u8, 0, 0x11, 0x22, 0x44, 0x88, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0],
                "/wARIkSIqrvM3e7/AA=="
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
