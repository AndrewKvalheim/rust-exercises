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
            .map(|c| c.to_digit(10).ok_or_else(|| InvalidDigit(c)))
            .try_fold(
                (None, VecDeque::with_capacity(length)),
                |(mut largest, mut series), digit| {
                    series.push_back(digit?.into());

                    if series.len() == length {
                        let product = series.iter().product();

                        series.pop_front();

                        // Pending RFC 1303, eRFC 2497
                        match largest {
                            Some(l) if l >= product => (),
                            _ => {
                                largest.replace(product);
                            }
                        }
                    }

                    Ok((largest, series))
                },
            )
            .map(|(largest, _)| largest.unwrap_or(1))
    }
}
