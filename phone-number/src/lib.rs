pub fn number(input: &str) -> Option<String> {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .enumerate()
        .filter(|&(i, c)| !(i == 0 && c == '1'))
        .enumerate()
        .map(|(i, (_, c))| match c {
            '0'...'1' if [0, 3].contains(&i) => None,
            _ => Some(c),
        })
        .collect::<Option<String>>()
        .filter(|s| s.len() == 10)
}
