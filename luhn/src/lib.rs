pub fn is_valid(numeral: &str) -> bool {
    numeral
        .chars()
        .rev()
        .filter(|character| !character.is_whitespace())
        .map(|character| character.to_digit(10))
        .enumerate()
        .try_fold((0, 0), |(length, sum), (index, parsed_digit)| {
            parsed_digit
                .map(|digit| match index % 2 {
                    0 => digit,
                    _ if digit < 5 => digit * 2,
                    _ => digit * 2 - 9,
                }).map(|value| (length + 1, sum + value))
        }).filter(|&(length, sum)| length > 1 && sum % 10 == 0)
        .is_some()
}
