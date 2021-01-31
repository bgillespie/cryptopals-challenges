//! All bytes in the ciphertext have been XOR'd with a single value.
//! Find the value and decipher it.

use crate::challenges::bytes::hex_to_bytes;
use crate::challenges::utils::most_popular;

type Rezult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn solution(ciphertext: &str) -> Rezult<String> {
    let bytez = hex_to_bytes(ciphertext)?;

    // Find the most popular item, which we reckon is space " " XOR'd with the
    // key we're looking for.
    let popular = most_popular(&bytez)[0];

    // XORing the code for space -- 32 -- we get the key.
    let key = popular ^ 32 as u8;

    // Now just XOR all bytes in the original ciphertext with the key, convert
    // to a String and return it.
    Ok(bytez.iter().map(|i| char::from(*i ^ key)).collect())
}

pub fn main() -> Rezult<()> {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let cracked = solution(ciphertext)?;
    println!("{}", cracked);
    Ok(())
}
