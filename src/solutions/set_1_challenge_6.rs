//! Break repeating-key XOR

use std::{fs::File, io::Read};

use thiserror;

use crate::challenges::bytes::hamming_distance;

type Rezult<T> = Result<T, Box<dyn std::error::Error>>;

// #[derive(Debug, thiserror::Error)]
// pub enum CompareError {
//     #[error("Not enough data to do the compare as specified")]
//     NotEnoughData,

//     #[error("Need more blocks to compare: must be >1")]
//     NeedMoreBlocks,

//     #[error("Need a bigger key size, must be >0")]
//     NeedBiggerKeySize,
// }

// pub fn compare_blocks(source: &[u8], keysize: usize, num_blocks: usize) -> Rezult<f32> {
//     if source.len() < keysize * num_blocks {
//         return Err(Box::new(CompareError::NotEnoughData));
//     }
//     if keysize < 1 {
//         return Err(Box::new(CompareError::NeedBiggerKeySize));
//     }
//     if num_blocks < 2 {
//         return Err(Box::new(CompareError::NeedBiggerKeySize))
//     }
//     (0..num_blocks).map(|block| block * keysize).map(|offset| hamming_distance(source[offset], b))
//     Ok(0.0)
// }

// pub fn solution(source: &[u8]) -> Rezult<String> {
//     const MIN_KEY_SIZE: usize = 2;
//     const MAX_KEY_SIZE: usize = 40;

//     let mut keysize_editsizes: Vec<(f32, usize)> = vec![];
//     let mut editsize: f32;
//     for keysize in MIN_KEY_SIZE..=MAX_KEY_SIZE {
//         editsize = hamming_distance(
//             &source[0..keysize],
//             &source[keysize..keysize * 2]
//         )? as f32;
//         editsize /= keysize as f32;
//         keysize_editsizes.push((editsize, keysize));
//     }
//     keysize_editsizes.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());
//     let probably_keysize = keysize_editsizes[0].1;
//     Ok("".into())
// }

// pub fn main() -> Rezult<()> {
//     let mut source = String::new();
//     File::open("data/6.txt")?.read_to_string(&mut source)?;
//     let result = solution(&source.as_bytes())?;
//     println!("Got it: {}", result);
//     Ok(())
// }
