mod iter_primes;
mod ord_by_item;

use iter_primes::IterPrimes;

pub fn primes_up_to(limit: u64) -> Vec<u64> {
    IterPrimes::new().take_while(|&p| p <= limit).collect()
}
