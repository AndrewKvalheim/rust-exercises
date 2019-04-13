mod iter_primes;
mod ord_by_item;

use iter_primes::IterPrimes;

pub fn nth(n: u32) -> u32 {
    IterPrimes::new().nth(n as usize).unwrap()
}
