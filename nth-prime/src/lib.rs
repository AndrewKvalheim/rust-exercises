use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::StepBy;
use std::ops::RangeFrom;

mod ord_by_item;

use ord_by_item::OrdByItem;

pub fn nth(n: u32) -> u32 {
    Primes::new().nth(n as usize).unwrap()
}

// Adaptation of the priority queue–based algorithm from The Genuine Sieve of Eratosthenes
// <https://doi.org/10.1017/S0956796808007004>
struct Primes {
    compositeses: BinaryHeap<Reverse<OrdByItem<StepBy<RangeFrom<u32>>>>>,
    naturals: RangeFrom<u32>,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            compositeses: BinaryHeap::new(),
            naturals: 2..,
        }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let compositeses = &mut self.compositeses;

        self.naturals
            .find(|&n| match compositeses.peek().map(|c| c.0.item) {
                Some(Some(composite)) if composite == n => {
                    while let Some(mut composites) = compositeses.peek_mut() {
                        if composites.0.item != Some(n) {
                            break;
                        }

                        composites.0.next();
                    }

                    false
                }
                _ => {
                    compositeses.push(Reverse(OrdByItem::new(
                        (u64::from(n).pow(2) as u32..).step_by(n as usize),
                    )));

                    true
                }
            })
    }
}
