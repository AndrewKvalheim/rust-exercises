pub fn rotate(text: &str, key: u8) -> String {
    let substitute = |start, c| char::from(start + (c as u8 - start + key) % 26);

    text.chars()
        .map(|character| match character {
            'A'...'Z' => substitute(b'A', character),
            'a'...'z' => substitute(b'a', character),
            _ => character,
        })
        .collect()
}
