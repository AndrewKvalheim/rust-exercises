pub fn get_diamond(letter: char) -> Vec<String> {
    let peak = letter as i8 - b'A' as i8;
    let edge = || (-peak..=peak).map(|i| i.abs());

    edge()
        .map(|x| {
            edge()
                .map(|y| if x + y == peak { char::from(b'A' + y as u8) } else { ' ' })
                .collect()
        })
        .collect()
}
