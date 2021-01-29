use std::fs;
use std::io::{self, BufRead};
use std::path;
///! Some general utils
use std::{collections::HashMap, hash::Hash};

/// Load lines from a file into a `<Vec<String>>`
pub fn load_lines<P>(path: P) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    P: AsRef<path::Path>,
{
    Ok(io::BufReader::new(fs::File::open(path)?)
        .lines()
        .collect::<Result<Vec<String>, _>>()?)
}

/// Trim lines of a `Vec<String>` by modifying in place.
pub fn trim_lines(lines: &mut Vec<String>) {
    for line in lines {
        *line = line.trim().to_string();
    }
}

/// Discard empty lines by modifying a `Vec<String>` in place.
pub fn discard_empty_lines(lines: &mut Vec<String>) {
    for i in (0..lines.len()).rev() {
        if lines[i].len() == 0 {
            lines.remove(i);
        }
    }
}

/// String contains only printable ASCII characters.
pub fn is_printable_ascii(s: &str) -> bool {
    s.as_bytes().iter().all(|b| *b >= 32 && *b <= 126)
}

/// Count the frequency of each item in a collection.
///
pub fn frequencies<T>(things: &[T]) -> HashMap<&T, usize>
where
    T: Eq + Hash,
{
    let mut counts = HashMap::new();
    things
        .iter()
        .for_each(|e| *counts.entry(e).or_insert(0) += 1);
    counts
}

/// Get pointers to items in the given collection, in the order of most common first.
///
pub fn most_popular<T>(things: &[T]) -> Vec<&T>
where
    T: Eq + Hash + Ord,
{
    let freqs = frequencies(things);
    let mut popular = freqs
        .iter()
        .map(|(&k, &v)| (v, k))
        .collect::<Vec<(usize, &T)>>();
    popular.sort();
    popular.reverse();
    popular.iter().map(|p| p.1).collect()
}

#[cfg(test)]
mod test {
    use super::{discard_empty_lines, frequencies, most_popular};
    use maplit::hashmap;

    #[test]
    fn test_discard_empty_lines() {
        let tests = [
            "hello\nworld\n!\n",
            "\nhello\nworld\n!",
            "hello\n\nworld\n\n!\n",
            "hello\nworld\n!",
        ];
        let expected = "hello\nworld\n!"
            .split('\n')
            .map(str::to_string)
            .collect::<Vec<String>>();
        for test in tests.iter() {
            let mut actual = test
                .split('\n')
                .map(str::to_string)
                .collect::<Vec<String>>();
            discard_empty_lines(&mut actual);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_frequencies() {
        let tests = [
            (vec![], hashmap! {}),
            (
                vec![0u8, 1, 2, 3],
                hashmap! {&0u8 => 1usize, &1 => 1, &2 => 1, &3 => 1},
            ),
            (vec![0u8, 1, 1, 0], hashmap! {&0u8 => 2, &1 => 2}),
        ];
        for (source, expected) in tests.iter() {
            let actual = frequencies(source);
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn test_most_popular() {
        let tests = [(vec![], vec![]), (vec![1, 1, 1, 2, 2, 3], vec![&1, &2, &3])];
        for (source, expected) in tests.iter() {
            let actual = most_popular(source);
            assert_eq!(expected, &actual);
        }
    }
}
