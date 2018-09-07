pub fn series(numeral: &str, length: usize) -> Vec<String> {
    (0..(1 + numeral.len() - length))
        .map(|start| numeral[start..start + length].into())
        .collect()
}
