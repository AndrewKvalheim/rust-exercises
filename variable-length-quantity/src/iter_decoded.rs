use Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub struct IterDecoded<'a, I: Iterator<Item = &'a u8>> {
    accumulated: Option<u32>,
    iter_octets: I,
}

impl<'a, I: Iterator<Item = &'a u8>> IterDecoded<'a, I> {
    pub fn new(iter_octets: I) -> Self {
        Self {
            accumulated: None,
            iter_octets,
        }
    }
}

impl<'a, I: Iterator<Item = &'a u8>> std::iter::Iterator for IterDecoded<'a, I> {
    type Item = Result<u32, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&octet) = self.iter_octets.next() {
            self.accumulated = self
                .accumulated
                .unwrap_or(0)
                .checked_mul(1 << 7)
                .and_then(|a| a.checked_add(u32::from(octet & 0b0111_1111)));

            if octet & (1 << 7) == 0 {
                return Some(self.accumulated.take().ok_or(Overflow));
            }
        }

        self.accumulated.map(|_| Err(IncompleteNumber))
    }
}
