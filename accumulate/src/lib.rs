pub fn map<F, T, U>(items: Vec<T>, mut transform: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut transformed = Vec::with_capacity(items.len());

    for item in items {
        transformed.push(transform(item));
    }

    transformed
}
