///! Some general utils

use std::{collections::HashMap, hash::Hash};

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
    use super::{frequencies, most_popular};
    use maplit::hashmap;

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
        let tests = [
            (vec![], vec![]),
            (vec![1, 1, 1, 2, 2, 3], vec![&1, &2, &3]),
        ];
        for (source, expected) in tests.iter() {
            let actual = most_popular(source);
            assert_eq!(expected, &actual);
        }
    }
}


