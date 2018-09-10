pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|&character| character != '-')
        .enumerate()
        .flat_map(|indexed_numeral| match indexed_numeral {
            (9, 'X') => Some(1),
            (index, numeral) => numeral.to_digit(10).map(|d| d * (1 + index as u32)),
        })
        .try_fold((0, 0), |(count, sum), digit| Some((count + 1, sum + digit)))
        .filter(|&(count, sum)| count == 10 && sum % 11 == 0)
        .is_some()
}
