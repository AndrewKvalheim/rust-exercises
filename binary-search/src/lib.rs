use std::cmp::Ordering::*;

pub fn find<T: Ord, Ts: AsRef<[T]>>(items: Ts, item: T) -> Option<usize> {
    let items = items.as_ref();
    let index = items.len() / 2;

    match item.cmp(items.get(index)?) {
        Equal => Some(index),
        Greater => find(&items[index + 1..], item).map(|f| index + 1 + f),
        Less => find(&items[..index], item),
    }
}
