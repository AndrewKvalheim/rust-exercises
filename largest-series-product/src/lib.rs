use std::collections::VecDeque;
use Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDigit(char),
    SpanTooLong,
}

pub fn lsp(numeral: &str, length: usize) -> Result<u64, Error> {
    if length > numeral.len() {
        Err(SpanTooLong)
    } else {
        numeral
            .chars()
            .try_fold(
                (None, VecDeque::with_capacity(length)),
                |(largest, mut series), character| {
                    let digit = character
                        .to_digit(10)
                        .ok_or_else(|| InvalidDigit(character))?
                        .into();

                    series.push_back(digit);

                    let next = if series.len() == length {
                        let product = series.iter().product();

                        series.pop_front();

                        match largest {
                            Some(l) if l >= product => largest,
                            _ => Some(product),
                        }
                    } else {
                        largest
                    };

                    Ok((next, series))
                },
            )
            .map(|(largest, _)| largest.unwrap_or(1))
    }
}
