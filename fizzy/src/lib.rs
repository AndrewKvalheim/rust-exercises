#![feature(trait_alias)]

mod fizzy;
mod matcher;

pub use crate::fizzy::Fizzy;
pub use crate::matcher::Matcher;
use std::ops::{Add, Rem};

pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: 'a + Add + From<u8> + Rem,
    <T as Rem>::Output: From<u8> + PartialEq,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}
