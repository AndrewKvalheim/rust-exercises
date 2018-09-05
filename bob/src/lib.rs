#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref MESSAGE_PATTERN: Regex = Regex::new(
        r"(?sx)^

            # Optional whitespace
            \s*

            # Message
            (?:
                # Shouting
                (?P<shouting> [^\p{Lowercase}]*? \p{Uppercase} [^\p{Lowercase}]*? )

                # or non-shouting
                | .*? \S .*?
            )

            # Optional question mark
            (?P<question>\?)?

            # Optional whitespace
            \s*

            $",
    )
    .unwrap();
}

pub fn reply(message: &str) -> &str {
    match MESSAGE_PATTERN.captures(message) {
        None => "Fine. Be that way!",
        Some(captures) => match (captures.name("shouting"), captures.name("question")) {
            (None, None) => "Whatever.",
            (None, Some(_)) => "Sure.",
            (Some(_), None) => "Whoa, chill out!",
            (Some(_), Some(_)) => "Calm down, I know what I'm doing!",
        },
    }
}
