use std::cmp;

pub struct IterMoves {
    amounts: (u8, u8),
    capacities: (u8, u8),
    goal: u8,
}

impl IterMoves {
    pub fn new(goal: u8, capacities: (u8, u8)) -> Self {
        IterMoves {
            amounts: (0, 0),
            capacities,
            goal,
        }
    }
}

impl std::iter::Iterator for IterMoves {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.amounts.0 == 0 {
            self.amounts.0 = self.capacities.0;
        } else if self.capacities.1 == self.goal {
            self.amounts.1 = self.capacities.1;
        } else if self.amounts.1 == self.capacities.1 {
            self.amounts.1 = 0;
        } else {
            let poured = cmp::min(self.amounts.0, self.capacities.1 - self.amounts.1);

            self.amounts.0 -= poured;
            self.amounts.1 += poured;
        }

        Some(self.amounts)
    }
}
