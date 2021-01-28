mod challenges;

use std::error::Error;

use challenges::utils::most_popular;
use challenges::bytes::hex_to_bytes;

/// All bytes in the ciphertext have been XOR'd with a single value.
/// Find the value and decipher it.
fn set_1_challenge_3(ciphertext: &str) -> Result<String, Box<dyn Error>> {
    let bytez = hex_to_bytes(ciphertext)?;
    
    // Find the most popular item, which we reckon is space " " XOR'd with the
    // key we're looking for.
    let popular = most_popular(&bytez)[0];

    // XORing the code for space -- 32 -- we get the key.
    let key = popular ^ 32 as u8;

    // Now just XOR all bytes in the original ciphertext with the key, convert
    // to a String and return it.    
    Ok(bytez
        .iter()
        .map(|i| char::from(*i ^ key))
        .collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let cracked = set_1_challenge_3(ciphertext)?;
    println!("{}", cracked);
    Ok(())
}
