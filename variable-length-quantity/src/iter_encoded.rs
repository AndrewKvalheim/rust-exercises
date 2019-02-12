pub struct IterEncoded<'a, I: Iterator<Item = &'a u32>> {
    iter_values: I,
    remaining: u32,
}

impl<'a, I: Iterator<Item = &'a u32>> IterEncoded<'a, I> {
    pub fn new(iter_values: I) -> Self {
        Self {
            iter_values,
            remaining: 0,
        }
    }
}

impl<'a, I: Iterator<Item = &'a u32>> std::iter::Iterator for IterEncoded<'a, I> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let continued = self.remaining != 0;

        let value = if continued {
            self.remaining
        } else {
            *self.iter_values.next()?
        };

        self.remaining = value >> 7;

        Some((value & 0b0111_1111) as u8 | if continued { 1 << 7 } else { 0 })
    }
}
