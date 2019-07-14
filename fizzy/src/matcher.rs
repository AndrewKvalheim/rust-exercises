pub trait Predicate<T> = Fn(T) -> bool;

pub struct Matcher<'a, T> {
    predicate: Box<'a + Predicate<T>>,
    word: String,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<F: 'a + Predicate<T>>(predicate: F, word: &str) -> Self {
        Self {
            predicate: Box::new(predicate),
            word: word.to_owned(),
        }
    }

    pub fn matching_word(&self, value: T) -> Option<&str> {
        if (self.predicate)(value) {
            Some(&self.word)
        } else {
            None
        }
    }
}
