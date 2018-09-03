const CODE: &[(usize, &str)] = &[(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(number: usize) -> String {
    let sound = CODE
        .iter()
        .filter_map(|&(f, p)| if number % f == 0 { Some(p) } else { None })
        .collect::<String>();

    if sound.is_empty() {
        number.to_string()
    } else {
        sound
    }
}
