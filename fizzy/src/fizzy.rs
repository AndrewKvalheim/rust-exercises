use crate::matcher::Matcher;

#[derive(Default)]
pub struct Fizzy<'a, T> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T> Fizzy<'a, T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);

        self
    }
}

impl<T: Copy + ToString> Fizzy<'_, T> {
    pub fn apply<'a, I>(&'a self, values: I) -> impl 'a + Iterator<Item = String>
    where
        I: 'a + Iterator<Item = T>,
    {
        values.map(move |v| self.word_for(v))
    }

    fn word_for(&self, value: T) -> String {
        let compound_word = self
            .matchers
            .iter()
            .filter_map(|m| m.matching_word(value))
            .collect::<String>();

        if compound_word.is_empty() {
            value.to_string()
        } else {
            compound_word
        }
    }
}
