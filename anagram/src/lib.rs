mod letter_counts;
mod normalized_word;

use letter_counts::LetterCounts;
use normalized_word::NormalizedWord;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let word = NormalizedWord::from(word);
    let letter_counts = LetterCounts::from(&word);

    candidates
        .iter()
        .filter(|&&candidate| {
            let candidate = NormalizedWord::from(candidate);

            candidate != word && LetterCounts::from(&candidate) == letter_counts
        })
        .copied()
        .collect()
}
