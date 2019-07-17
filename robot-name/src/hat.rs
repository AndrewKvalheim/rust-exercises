use rand::prelude::*;
use std::iter::FromIterator;

pub struct Hat<T>(Vec<T>);

impl<T> Hat<T> {
    pub fn draw(&mut self) -> Option<T> {
        match self.0.len() {
            0 => None,
            1 => self.0.pop(),
            l => Some(self.0.swap_remove(thread_rng().gen_range(0, l))),
        }
    }

    pub fn insert(&mut self, item: T) {
        self.0.push(item);
    }
}

impl<T> FromIterator<T> for Hat<T> {
    fn from_iter<I: IntoIterator<Item = T>>(items: I) -> Self {
        Self(items.into_iter().collect())
    }
}
