use std::iter;
use std::num::NonZeroU64;

pub fn collatz(initial: u64) -> Option<u64> {
    let steps = iter::successors(NonZeroU64::new(initial).map(NonZeroU64::get), |n| match n {
        1 => None,
        _ => Some(if n % 2 == 0 { n / 2 } else { 3 * n + 1 }),
    });

    (steps.count() as u64).checked_sub(1)
}
