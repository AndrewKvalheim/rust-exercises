pub trait CollectionUtilities<'a> {
    fn extend_from_strs<T: IntoIterator<Item = &'a (&'a str, &'a str)>>(&mut self, iter: T);
}

impl<'a, C> CollectionUtilities<'a> for C
where
    C: Extend<(String, String)>,
{
    fn extend_from_strs<T: IntoIterator<Item = &'a (&'a str, &'a str)>>(&mut self, iter: T) {
        self.extend(
            iter.into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
    }
}
