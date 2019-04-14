use std::iter;

pub fn triplets(text: &str) -> impl Iterator<Item = String> + '_ {
    let mut chars = text.chars();

    iter::from_fn(move || Some([chars.next()?, chars.next()?, chars.next()?].iter().collect()))
}
