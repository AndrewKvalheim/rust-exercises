use std::convert::TryFrom;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct Radix(u32);

impl Deref for Radix {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<u32> for Radix {
    type Error = u32;

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n > 1 {
            Ok(Radix(n))
        } else {
            Err(n)
        }
    }
}
