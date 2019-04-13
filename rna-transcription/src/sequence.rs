use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Sequence<T>(pub Vec<T>);

impl<'a, T: 'a> Sequence<T> {
    pub fn to_base<U: From<&'a T>>(&'a self) -> Sequence<U> {
        Sequence(self.0.iter().map(|b| b.into()).collect())
    }
}

impl<T: TryFrom<char>> Sequence<T> {
    // Spec contradicts clippy::new_ret_no_self.
    pub fn new(text: &str) -> Result<Self, usize> {
        Self::try_from(text)
    }
}

impl<T: TryFrom<char>> TryFrom<&str> for Sequence<T> {
    type Error = usize;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        text.chars()
            .enumerate()
            .map(|(i, c)| T::try_from(c).map_err(|_| i))
            .collect::<Result<_, _>>()
            .map(Sequence)
    }
}
