mod number;
mod radix;
mod rfc_1542;
mod utilities;

use number::Number;
use radix::Radix;
use rfc_1542::TryFrom;
use utilities::reverse;
use Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDigit(u32),
    InvalidInputBase,
    InvalidOutputBase,
}

pub fn convert(from_digits: &[u32], from_radix: u32, to_radix: u32) -> Result<Vec<u32>, Error> {
    let from_radix = Radix::try_from(from_radix).map_err(|_| InvalidInputBase)?;
    let to_radix = Radix::try_from(to_radix).map_err(|_| InvalidOutputBase)?;

    let digits = Number::try_from_digits(from_radix, from_digits.iter().rev())
        .map_err(InvalidDigit)?
        .digits(to_radix)
        .collect();

    Ok(reverse(digits))
}
