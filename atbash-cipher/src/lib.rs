mod utilities;

use utilities::IteratorUtilities;

pub fn decode(ciphertext: &str) -> String {
    substitutions(ciphertext).collect()
}

pub fn encode(plaintext: &str) -> String {
    substitutions(plaintext).insert_every(5, ' ').collect()
}

fn substitutions(text: &str) -> impl '_ + Iterator<Item = char> {
    text.chars().filter_map(|character| {
        if character.is_ascii_alphabetic() {
            Some((b'a' + b'z' - character.to_ascii_lowercase() as u8) as char)
        } else if character.is_numeric() {
            Some(character)
        } else {
            None
        }
    })
}
