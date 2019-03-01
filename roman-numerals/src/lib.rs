use std::fmt::{self, Display};
use std::iter;

const DIGITS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman(u32);

impl Roman {
    fn digits(&self) -> impl Iterator<Item = &str> {
        DIGITS
            .iter()
            .scan(self.0, |rest, &(value, symbol)| {
                let count = *rest / value;

                *rest %= value;

                Some(iter::repeat(symbol).take(count as usize))
            })
            .flatten()
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.digits().map(|d| f.write_str(d)).collect()
    }
}

impl From<u32> for Roman {
    fn from(n: u32) -> Self {
        Self(n)
    }
}
