use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::Sized;

pub trait IteratorUtilities: Iterator {
    fn unique(self) -> Unique<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        Unique::new(self)
    }
}

impl<I: Iterator> IteratorUtilities for I {}

pub struct Unique<I: Iterator> {
    iterator: I,
    seen: HashSet<I::Item>,
}

impl<I> Unique<I>
where
    I: Iterator,
    I::Item: Eq + Hash,
{
    fn new(iterator: I) -> Self {
        Self {
            iterator,
            seen: HashSet::new(),
        }
    }
}

impl<I> Iterator for Unique<I>
where
    I: Iterator,
    I::Item: Clone + Eq + Hash,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.iterator.next() {
            if self.seen.insert(next.clone()) {
                return Some(next);
            }
        }

        None
    }
}
