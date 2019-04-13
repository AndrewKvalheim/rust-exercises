pub struct IterDigits(pub u32);

impl Iterator for IterDigits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let digit = self.0 % 10;

            self.0 /= 10;

            Some(digit)
        }
    }
}

impl ExactSizeIterator for IterDigits {
    fn len(&self) -> usize {
        (1.0 + f64::from(self.0).log10()).floor() as usize
    }
}
