#[derive(Debug, PartialEq)]
pub enum FixedXorError {
    SourcesDifferingLength,
}

pub fn fixed_xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, FixedXorError> {
    if a.len() != b.len() {
        Err(FixedXorError::SourcesDifferingLength)
    }
    else {
        Ok(a.iter().zip(b.iter()).map(|(l, r)| *l ^ *r).collect())
    }
}

#[cfg(test)]
mod test {
    use crate::challenges::utils::hex_to_bytes;

    use super::fixed_xor;

    #[test]
    fn test_fixed_xor() {
        let tests = [
            ("", "", ""),
            (
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965",
                "746865206b696420646f6e277420706c6179"
            )
        ];
        for &(a, b, expected) in tests.iter() {
            let a = hex_to_bytes(a).unwrap();
            let b = hex_to_bytes(b).unwrap();
            let expected = hex_to_bytes(expected).unwrap();
            let actual = fixed_xor(&a, &b);
            assert_eq!(Ok(expected), actual);
        }
    }
}