pub fn rotate(text: &str, key: u8) -> String {
    let substitute = |start, end, c| (start + (c as u8 - start + key) % (end - start + 1)) as char;

    text.chars()
        .map(|character| match character {
            'A'...'Z' => substitute(b'A', b'Z', character),
            'a'...'z' => substitute(b'a', b'z', character),
            _ => character,
        })
        .collect()
}
