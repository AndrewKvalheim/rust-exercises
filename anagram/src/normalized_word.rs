#[derive(PartialEq)]
pub struct NormalizedWord(Vec<String>);

impl NormalizedWord {
    pub fn letters(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }
}

impl From<&str> for NormalizedWord {
    fn from(word: &str) -> Self {
        Self(word.chars().map(|c| c.to_lowercase().collect()).collect())
    }
}
