pub fn abbreviate(term: &str) -> String {
    term.chars()
        .scan(None::<char>, |previous, character| {
            let letter = match previous {
                None => Some(character),
                Some(p) if !p.is_alphabetic() && character.is_alphabetic() => Some(character),
                Some(p) if p.is_lowercase() && character.is_uppercase() => Some(character),
                _ => None,
            };

            *previous = Some(character);

            Some(letter)
        })
        .filter_map(/* Pending RFC 2306 */ |l| l)
        .flat_map(char::to_uppercase)
        .collect()
}
