//! Repeating-key XOR

type Rezult<T> = Result<T, Box<dyn std::error::Error>>;

use crate::challenges::bytes::{bytes_to_hex, repeating_key_xor};

/// Do repeating-key XOR on an input text.
pub fn solution(input: &str, key: &str) -> String {
    bytes_to_hex(&repeating_key_xor(
        input.trim().as_bytes(),
        key.trim().as_bytes(),
        0,
    ))
}

pub fn main() -> Rezult<()> {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let key = "ICE";
    let output = solution(input, key);
    if &output == expected {
        println!("Win! ");
    } else {
        println!("FAIL ");
    }
    println!("Got      : {}", output);
    println!("Expected : {}", expected);
    Ok(())
}
