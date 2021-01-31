//! Detect single-character XOR.
//! Find the string in the file that has been single-char XOR'd.

type Rezult<T> = Result<T, Box<dyn std::error::Error>>;

use crate::challenges::utils::{discard_empty_lines, is_printable_ascii, load_lines, trim_lines};

use super::set_1_challenge_3;

/// There's one line in a bunch of lines that can be found using logic from the
/// previous challenge: find it.
pub fn solution(lines: &mut Vec<String>) -> Rezult<Option<(usize, String)>> {
    for (i, line) in lines.iter().enumerate() {
        let attempt = set_1_challenge_3::solution(&line)?;
        if is_printable_ascii(attempt.trim()) {
            return Ok(Some((i, attempt)));
        }
    }
    Ok(None)
}

pub fn main() -> Rezult<()> {
    let mut lines = load_lines("data/4.txt")?;
    trim_lines(&mut lines);
    discard_empty_lines(&mut lines);
    let result = solution(&mut lines)?;
    if let Some((line_no, decrypted)) = result {
        println!("FOUND! At line {}, {}", line_no, decrypted);
    } else {
    }
    Ok(())
}
