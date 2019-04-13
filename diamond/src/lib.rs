pub fn get_diamond(letter: char) -> Vec<String> {
    let peak = letter as i8 - b'A' as i8;
    let edge = || (-peak..=peak).map(i8::abs);
    let letter = |i| char::from(b'A' + i as u8);

    edge().map(|x| {
        edge().map(|y| if x + y == peak { letter(y) } else { ' ' })
            .collect()
    })
    .collect()
}
