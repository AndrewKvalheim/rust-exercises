use crate::normalized_word::NormalizedWord;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(PartialEq)]
pub struct LetterCounts(HashMap<String, u32>);

impl LetterCounts {
    fn insert(&mut self, letter: String) {
        *self.0.entry(letter).or_insert(0) += 1;
    }

    fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Extend<String> for LetterCounts {
    fn extend<I: IntoIterator<Item = String>>(&mut self, letters: I) {
        for letter in letters {
            self.insert(letter);
        }
    }
}

impl From<&NormalizedWord> for LetterCounts {
    fn from(word: &NormalizedWord) -> Self {
        word.letters().cloned().collect()
    }
}

impl FromIterator<String> for LetterCounts {
    fn from_iter<I: IntoIterator<Item = String>>(letters: I) -> Self {
        let mut counts = Self::new();

        counts.extend(letters);

        counts
    }
}
