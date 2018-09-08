use std::collections::HashSet;

pub fn check(word: &str) -> bool {
    let mut letters = HashSet::new();

    word.chars()
        .filter(|&character| character.is_alphabetic())
        .find(|&letter| !letters.insert(letter.to_lowercase().collect::<String>()))
        .is_none()
}
