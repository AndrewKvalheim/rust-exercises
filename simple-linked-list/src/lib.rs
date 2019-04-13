// Pending rust-lang-nursery/rust-clippy#2226
#![cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default_derive))]

type Next<T> = Option<Box<Node<T>>>;

struct Node<T> {
    next: Next<T>,
    value: T,
}

pub struct SimpleLinkedList<T> {
    head: Next<T>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn peek(&self) -> Option<&T> {
        Some(&self.head.as_ref()?.value)
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = *self.head.take()?;

        self.head = node.next;
        self.length -= 1;

        Some(node.value)
    }

    pub fn push(&mut self, value: T) {
        let next = self.head.take();

        self.head = Some(Box::new(Node { next, value }));
        self.length += 1;
    }

    pub fn rev(self) -> Self {
        self.into_iter().collect()
    }
}

//
// extend
//

impl<T> Extend<T> for SimpleLinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterator: I) {
        for item in iterator {
            self.push(item);
        }
    }
}

//
// from
//

impl<T: Clone> From<&[T]> for SimpleLinkedList<T> {
    fn from(slice: &[T]) -> Self {
        slice.iter().collect()
    }
}

//
// from_iter
//

impl<T> std::iter::FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iterator: I) -> Self {
        let mut list = Self::new();

        list.extend(iterator);

        list
    }
}

impl<'a, T: 'a + Clone> std::iter::FromIterator<&'a T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = &'a T>>(iterator: I) -> Self {
        let mut list = Self::new();

        list.extend(iterator.into_iter().cloned());

        list
    }
}

//
// into
//

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vector = Vec::new();

        vector.extend(self);
        vector.reverse();

        vector
    }
}

//
// into_iter
//

impl<T> IntoIterator for SimpleLinkedList<T> {
    type IntoIter = SimpleLinkedListIntoIterator<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        // Pending RFC 2338
        SimpleLinkedListIntoIterator(self)
    }
}

pub struct SimpleLinkedListIntoIterator<T>(SimpleLinkedList<T>);

impl<T> ExactSizeIterator for SimpleLinkedListIntoIterator<T> {}

impl<T> Iterator for SimpleLinkedListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.length, Some(self.0.length))
    }
}

//
// iter
//

impl<T> SimpleLinkedList<T> {
    pub fn iter(&self) -> SimpleLinkedListIterator<T> {
        SimpleLinkedListIterator::new(self)
    }
}

pub struct SimpleLinkedListIterator<'a, T: 'a> {
    length: usize,
    next: Option<&'a Node<T>>,
}

impl<'a, T> SimpleLinkedListIterator<'a, T> {
    pub fn new(list: &'a SimpleLinkedList<T>) -> Self {
        Self {
            length: list.length,
            next: list.head.as_ref().map(|b| &**b),
        }
    }
}

impl<T> ExactSizeIterator for SimpleLinkedListIterator<'_, T> {}

impl<'a, T> Iterator for SimpleLinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.next?;

        self.length -= 1;
        self.next = node.next.as_ref().map(|b| &**b);

        Some(&node.value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}
