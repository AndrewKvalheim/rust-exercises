use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

// Wraps an iterator to allow comparison by its next item.
pub struct OrdByItem<I>
where
    I: Iterator,
{
    pub item: Option<I::Item>,
    iterator: I,
}

impl<I> OrdByItem<I>
where
    I: Iterator,
{
    pub fn new(mut iterator: I) -> Self {
        let item = iterator.next();

        OrdByItem { item, iterator }
    }

    pub fn next(&mut self) {
        self.item = self.iterator.next();
    }
}

impl<I> Eq for OrdByItem<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
}

impl<I> Ord for OrdByItem<I>
where
    I: Iterator,
    I::Item: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl<I> PartialEq for OrdByItem<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<I> PartialOrd for OrdByItem<I>
where
    I: Iterator,
    I::Item: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
