#[derive(PartialEq)]
pub struct NormalizedWord(Vec<String>);

impl NormalizedWord {
    pub fn letters(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
}

impl From<&str> for NormalizedWord {
    fn from(word: &str) -> Self {
        Self(word.chars().map(|c| c.to_lowercase().collect()).collect())
    }
}
