use crate::normalized_word::NormalizedWord;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(PartialEq)]
pub struct LetterCounts<'a>(HashMap<&'a str, u32>);

impl<'a> LetterCounts<'a> {
    fn insert(&mut self, letter: &'a str) {
        *self.0.entry(letter).or_insert(0) += 1;
    }

    fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<'a> Extend<&'a str> for LetterCounts<'a> {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, letters: I) {
        for letter in letters {
            self.insert(letter);
        }
    }
}

impl<'a> From<&'a NormalizedWord> for LetterCounts<'a> {
    fn from(word: &'a NormalizedWord) -> Self {
        word.letters().collect()
    }
}

impl<'a> FromIterator<&'a str> for LetterCounts<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(letters: I) -> Self {
        let mut counts = Self::new();

        counts.extend(letters);

        counts
    }
}
