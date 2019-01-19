pub trait Inspectable<T> {
    fn inspect<F: FnOnce(&T) -> ()>(self, f: F) -> Self;
}

impl<T> Inspectable<T> for std::io::Result<T> {
    fn inspect<F: FnOnce(&T) -> ()>(self, f: F) -> Self {
        self.map(|value| {
            f(&value);
            value
        })
    }
}
