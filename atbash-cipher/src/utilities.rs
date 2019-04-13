pub trait IteratorUtilities: Iterator {
    fn insert_every(self, period: usize, item: Self::Item) -> InsertEvery<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        InsertEvery::new(self, period, item)
    }
}

impl<I: Iterator> IteratorUtilities for I {}

pub struct InsertEvery<I: Iterator> {
    index: usize,
    item: I::Item,
    iterator: I,
    period: usize,
    postponed: Option<I::Item>,
}

impl<I: Iterator> InsertEvery<I> {
    fn new(iterator: I, period: usize, item: I::Item) -> Self {
        Self {
            index: 0,
            item,
            iterator,
            period: 1 + period,
            postponed: None,
        }
    }
}

impl<I: Iterator> Iterator for InsertEvery<I>
where
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        self.postponed.take().or_else(|| {
            if self.index % self.period == 0 {
                self.postponed = self.iterator.next();

                self.postponed.as_ref().map(|_| self.item.clone())
            } else {
                self.iterator.next()
            }
        })
    }
}
