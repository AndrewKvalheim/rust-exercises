use std::cmp::Ordering::{Equal, Greater, Less};
use Classification::{Abundant, Deficient, Perfect};

#[derive(Debug, PartialEq)]
pub enum Classification {
    Abundant,
    Deficient,
    Perfect,
}

pub fn classify(n: u64) -> Option<Classification> {
    Some(match aliquot_sum(n)?.cmp(&n) {
        Equal => Perfect,
        Greater => Abundant,
        Less => Deficient,
    })
}

fn aliquot_sum(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => Some((1..=n / 2).filter(|m| n % m == 0).sum()),
    }
}
