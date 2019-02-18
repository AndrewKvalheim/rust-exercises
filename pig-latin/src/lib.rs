#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(
        r"(?x)
            (?P<vowel>[xy][bcdfghjklmnpqrstvwxz]+)?
            (?P<consonant>y|(qu|[bcdfghjklmnpqrstvwxz])+)?
            (?P<rest>\w+)
        ",
    )
    .unwrap();
}

pub fn translate(phrase: &str) -> String {
    PATTERN
        .replace_all(phrase, "${vowel}${rest}${consonant}ay")
        .into()
}
