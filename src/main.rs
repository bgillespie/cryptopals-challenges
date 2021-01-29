mod challenges;

use std::error::Error;

use challenges::bytes::{bytes_to_hex, hex_to_bytes, repeating_key_xor};
use challenges::utils::{
    discard_empty_lines, is_printable_ascii, load_lines, most_popular, trim_lines,
};

type Rezult<T> = Result<T, Box<dyn Error>>;

/// All bytes in the ciphertext have been XOR'd with a single value.
/// Find the value and decipher it.
fn set_1_challenge_3_solution(ciphertext: &str) -> Rezult<String> {
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

fn set_1_challenge_3() -> Rezult<()> {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let cracked = set_1_challenge_3_solution(ciphertext)?;
    println!("{}", cracked);
    Ok(())
}

/// There's one line in a bunch of lines that can be found using logic from the
/// previous challenge: find it.
fn set_1_challenge_4_solution(lines: &mut Vec<String>) -> Rezult<Option<(usize, String)>> {
    trim_lines(lines);
    discard_empty_lines(lines);
    for (i, line) in lines.iter().enumerate() {
        let attempt = set_1_challenge_3_solution(&line)?;
        if is_printable_ascii(attempt.trim()) {
            return Ok(Some((i, attempt)));
        }
    }
    Ok(None)
}

fn set_1_challenge_4() -> Rezult<()> {
    let mut lines = load_lines("data/4.txt")?;
    let result = set_1_challenge_4_solution(&mut lines)?;
    if let Some((line_no, decrypted)) = result {
        println!("FOUND! At line {}, {}", line_no, decrypted);
    } else {
    }
    Ok(())
}

/// Do repeating-key XOR on an input text.
fn set_1_challenge_5_solution(input: &str, key: &str) -> String {
    bytes_to_hex(&repeating_key_xor(input.trim().as_bytes(), key.trim().as_bytes(), 0))
}

fn set_1_challenge_5() -> Rezult<()> {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let key = "ICE";
    let output = set_1_challenge_5_solution(input, key);
    if &output == expected {
        println!("Win! ");
    }
    else {
        println!("FAIL ");
    }
    println!("Got      : {}", output);
    println!("Expected : {}", expected);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    set_1_challenge_3()?;
    set_1_challenge_4()?;
    set_1_challenge_5()?;
    Ok(())
}
