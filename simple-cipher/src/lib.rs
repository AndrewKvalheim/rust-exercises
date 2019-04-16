use rand::{thread_rng, Rng};
use std::ops::{Add, Sub};

pub fn decode(key: &str, ciphertext: &str) -> Option<String> {
    substitute(key, ciphertext, u8::sub)
}

pub fn encode(key: &str, plaintext: &str) -> Option<String> {
    substitute(key, plaintext, u8::add)
}

pub fn encode_random(plaintext: &str) -> (String, String) {
    let key = generate_key(100);
    let ciphertext = encode(&key, plaintext).expect("failed to encode");

    (key, ciphertext)
}

fn generate_key(length: usize) -> String {
    let mut rng = thread_rng();

    (0..length)
        .map(|_| char::from(rng.gen_range(b'a', 1 + b'z')))
        .collect()
}

fn substitute<F>(key: &str, plaintext: &str, operation: F) -> Option<String>
where
    F: Fn(u8, u8) -> u8,
{
    plaintext
        .chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| {
            if c.is_ascii_lowercase() && k.is_ascii_lowercase() {
                Some((b'a' + operation(26 + c as u8 - b'a', k as u8 - b'a') % 26) as char)
            } else {
                None
            }
        })
        .collect::<Option<String>>()
        .filter(|s| !s.is_empty())
}
