use crate::radix::Radix;

pub struct Number(u32);

impl Number {
    pub fn digits(&self, radix: Radix) -> impl Iterator<Item = u32> {
        IterDigits(*radix, self.0)
    }

    pub fn try_from_digits<'a>(
        radix: Radix,
        digits: impl Iterator<Item = &'a u32>,
    ) -> Result<Self, u32> {
        digits
            .map(|&d| if d < *radix { Ok(d) } else { Err(d) })
            .enumerate()
            .try_fold(0, |n, (i, d)| Ok(n + d? * radix.pow(i as u32)))
            .map(Number)
    }
}

struct IterDigits(u32, u32);

impl Iterator for IterDigits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Pending RFC 2757
        if self.1 == 0 {
            None
        } else {
            let digit = self.1 % self.0;

            self.1 /= self.0;

            Some(digit)
        }
    }
}
