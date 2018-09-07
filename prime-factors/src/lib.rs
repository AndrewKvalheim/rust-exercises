pub fn factors(n: u64) -> Vec<u64> {
    match (2..=n).find(|factor| n % factor == 0) {
        None => vec![],
        Some(factor) => [vec![factor], factors(n / factor)].concat(),
    }
}
