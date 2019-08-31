#![feature(euclidean_division)]

pub fn rotate(text: &str, key: i8) -> String {
    let replace = |a: u8, c: char| char::from(a + ((c as u8 - a) as i8 + key).rem_euclid(26) as u8);

    text.chars()
        .map(|character| match character {
            'A'..='Z' => replace(b'A', character),
            'a'..='z' => replace(b'a', character),
            _ => character,
        })
        .collect()
}
