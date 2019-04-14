use crate::translation::Translation::{self, *};
use crate::utilities::triplets;
use std::collections::HashMap;

pub struct CodonsInfo<'a>(HashMap<&'a str, Translation<'a>>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&str> {
        self.0.get(codon).map(|t| t.into())
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&str>> {
        triplets(rna)
            .scan((), |_, codon| match self.0.get(codon.as_str()) {
                None => Some(None),
                Some(Protein(name)) => Some(Some(*name)),
                Some(Stop) => None,
            })
            .collect()
    }
}

impl<'a, I, T> From<I> for CodonsInfo<'a>
where
    I: IntoIterator<Item = (&'a str, T)>,
    T: Into<Translation<'a>>,
{
    fn from(rules: I) -> Self {
        Self(rules.into_iter().map(|(c, n)| (c, n.into())).collect())
    }
}
