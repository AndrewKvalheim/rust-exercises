use std::iter::{once, repeat};

pub fn decode(code: &str) -> String {
    code.chars()
        .scan(String::new(), |numeral, character| {
            let length = if character.is_numeric() {
                numeral.push(character);

                0
            } else {
                let number = numeral.parse().unwrap_or(1);

                *numeral = String::new();

                number
            };

            Some(repeat(character).take(length))
        })
        .flatten()
        .collect()
}

pub fn encode(text: &str) -> String {
    text.chars()
        .map(Some)
        .chain(once(None))
        .scan(None, |run, character| {
            let instruction = match (&run, character) {
                (Some((previous, _)), Some(c)) if *previous == c => None,
                (Some((previous, length)), _) => Some(match length {
                    1 => format!("{}", previous),
                    _ => format!("{}{}", length, previous),
                }),
                _ => None,
            };

            match (run, character) {
                (Some((previous, length)), Some(c)) if *previous == c => *length += 1,
                (r, Some(c)) => *r = Some((c, 1)),
                _ => (),
            };

            Some(instruction)
        })
        .flatten()
        .collect()
}
