use crate::ord_by_item::OrdByItem;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::StepBy;
use std::ops::RangeFrom;

// Priority queue–based algorithm from https://doi.org/10.1017/S0956796808007004
pub struct IterPrimes {
    compositeses: BinaryHeap<Reverse<OrdByItem<StepBy<RangeFrom<u64>>>>>,
    naturals: RangeFrom<u64>,
}

impl IterPrimes {
    pub fn new() -> Self {
        Self {
            compositeses: BinaryHeap::new(),
            naturals: 2..,
        }
    }
}

impl Iterator for IterPrimes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let compositeses = &mut self.compositeses;

        self.naturals.find(|&n| {
            let is_prime = compositeses.peek().map(|c| c.0.item) != Some(Some(n));

            if is_prime {
                compositeses.push(Reverse(OrdByItem::new((n.pow(2)..).step_by(n as usize))));
            } else {
                while let Some(mut composites) = compositeses.peek_mut() {
                    // Pending eRFC 2497
                    if composites.0.item != Some(n) {
                        break;
                    }

                    composites.0.next();
                }
            }

            is_prime
        })
    }
}
