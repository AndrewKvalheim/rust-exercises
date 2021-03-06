use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{Extend, FromIterator};

pub struct Tally<T>(HashMap<T, u32>);

impl<T: Eq + Hash> Tally<T> {
    fn new() -> Self {
        Tally(HashMap::new())
    }

    fn insert(&mut self, item: T) {
        *self.0.entry(item).or_insert(0) += 1;
    }
}

impl<T: Eq + Hash> Extend<T> for Tally<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterator: I) {
        for item in iterator {
            self.insert(item);
        }
    }
}

impl<T: Eq + Hash> FromIterator<T> for Tally<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iterator: I) -> Self {
        let mut tally = Self::new();

        tally.extend(iterator);

        tally
    }
}

impl<T: Eq + Hash> Into<HashMap<T, u32>> for Tally<T> {
    fn into(self) -> HashMap<T, u32> {
        self.0
    }
}
