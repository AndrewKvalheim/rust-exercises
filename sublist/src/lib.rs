use std::cmp::Ordering::*;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    let contains = |o: &[T], i: &[T]| i.is_empty() || o.windows(i.len()).any(|w| w == i);

    match a.len().cmp(&b.len()) {
        Equal if a == b => Comparison::Equal,
        Greater if contains(a, b) => Comparison::Superlist,
        Less if contains(b, a) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}
