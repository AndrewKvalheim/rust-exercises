mod iter_digits;

use iter_digits::IterDigits;

pub fn is_armstrong_number(number: u32) -> bool {
    let digits = IterDigits(number);
    let length = digits.len() as u32;

    digits.map(|d| d.pow(length)).sum::<u32>() == number
}
