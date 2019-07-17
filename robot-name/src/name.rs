use std::fmt::{self, Display};

#[derive(Clone, Copy)]
pub struct Name(u32);

impl Name {
    pub fn all() -> impl Iterator<Item = Self> {
        (0..(26 * 26 * 1000)).map(Self)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rest = self.0;

        write!(f, "{}", char::from(b'A' + (rest % 26) as u8))?;
        rest /= 26;

        write!(f, "{}", char::from(b'A' + (rest % 26) as u8))?;
        rest /= 26;

        write!(f, "{:03}", rest)
    }
}
