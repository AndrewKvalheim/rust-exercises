pub fn encrypt(text: &str) -> String {
    let characters = text
        .chars()
        .filter(|&c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect::<Vec<_>>();

    let (m, n) = {
        let ideal = (characters.len() as f64).sqrt();

        (ideal.ceil() as usize, ideal.floor() as usize)
    };

    let row = |i| {
        (0..n)
            .map(|j| characters.get(i + j * m).unwrap_or(&' '))
            .collect()
    };

    (0..m).map(row).collect::<Vec<String>>().join(" ")
}
