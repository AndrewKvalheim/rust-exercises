use std::collections::BTreeMap;

type LettersByValue = BTreeMap<i32, Vec<char>>;
type ValueByLetter = BTreeMap<char, i32>;

pub struct Rules(ValueByLetter);

impl From<&LettersByValue> for Rules {
    fn from(specification: &LettersByValue) -> Self {
        Rules(
            specification
                .iter()
                .flat_map(|(value, letters)| {
                    letters
                        .iter()
                        .map(char::to_ascii_lowercase)
                        .map(move |letter| (letter, *value))
                })
                .collect(),
        )
    }
}

impl From<Rules> for ValueByLetter {
    fn from(rules: Rules) -> ValueByLetter {
        rules.0
    }
}
