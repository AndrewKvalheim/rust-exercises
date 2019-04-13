use std::collections::HashSet;

pub fn check(word: &str) -> bool {
    let mut letters = HashSet::<String>::new();

    word.chars()
        .filter(|&c| c.is_alphabetic())
        .all(|l| letters.insert(l.to_lowercase().collect()))
}
