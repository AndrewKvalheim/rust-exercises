mod tally;

use std::collections::HashMap;
use tally::Tally;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    iter_words(text).collect::<Tally<_>>().into()
}

fn iter_words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !(c == '\'' || c.is_alphanumeric()))
        .filter(|&s| !s.is_empty())
        .map(|s| s.trim_matches('\'').to_lowercase())
}
