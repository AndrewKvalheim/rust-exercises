pub fn is_pangram(text: &str) -> bool {
    text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold([false; 26], |mut seen, letter| {
            seen[letter.to_ascii_uppercase() as usize - b'A' as usize] = true;

            seen
        })
        .iter()
        .all(|&b| b)
}
