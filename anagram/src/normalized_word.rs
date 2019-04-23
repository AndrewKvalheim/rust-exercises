use caseless::Caseless;
use unicode_normalization::UnicodeNormalization;

#[derive(PartialEq)]
pub struct NormalizedWord(String);

impl NormalizedWord {
    pub fn letters(&self) -> impl Iterator<Item = char> + '_ {
        self.0.chars()
    }
}

impl From<&str> for NormalizedWord {
    fn from(word: &str) -> Self {
        Self(
            // As in Caseless::compatibility_caseless_match
            word.chars()
                .nfd()
                .default_case_fold()
                .nfkd()
                .default_case_fold()
                .nfkd()
                .collect(),
        )
    }
}
