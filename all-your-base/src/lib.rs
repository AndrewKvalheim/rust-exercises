mod number;
mod radix;
mod utilities;

use number::Number;
use std::convert::TryInto;
use utilities::reverse;
use Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDigit(u32),
    InvalidInputBase,
    InvalidOutputBase,
}

pub fn convert(from_digits: &[u32], from_radix: u32, to_radix: u32) -> Result<Vec<u32>, Error> {
    let from_radix = from_radix.try_into().map_err(|_| InvalidInputBase)?;
    let to_radix = to_radix.try_into().map_err(|_| InvalidOutputBase)?;

    let to_digits = Number::try_from_digits(from_radix, from_digits.iter().rev())
        .map_err(InvalidDigit)?
        .digits(to_radix)
        .collect();

    Ok(reverse(to_digits))
}
