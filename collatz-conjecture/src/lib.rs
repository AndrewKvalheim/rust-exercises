pub fn collatz(n: u64) -> Option<u64> {
    n.checked_sub(1).map(|m| Collatz(m).count() as u64)
}

struct Collatz(u64);

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match 1 + self.0 {
            1 => None,
            n => {
                self.0 = if n % 2 == 0 { n / 2 } else { 3 * n + 1 } - 1;

                Some(n)
            }
        }
    }
}
