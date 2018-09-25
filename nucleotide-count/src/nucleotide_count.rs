#![cfg_attr(feature = "cargo-clippy", allow(clippy::implicit_hasher))]

use crate::nucleotide::Nucleotide;
use crate::nucleotide::Nucleotide::*;
use std::collections::HashMap;

pub struct NucleotideCount {
    a: usize,
    c: usize,
    g: usize,
    t: usize,
}

impl NucleotideCount {
    fn new() -> Self {
        Self {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
        }
    }
}

impl<'a> NucleotideCount {
    // Pending RFC 1542
    pub fn try_from(sequence: &'a str) -> Result<Self, char> {
        sequence.chars().map(Nucleotide::try_from).collect()
    }
}

impl Extend<Nucleotide> for NucleotideCount {
    fn extend<I: IntoIterator<Item = Nucleotide>>(&mut self, iterator: I) {
        for nucleotide in iterator {
            match nucleotide {
                A => self.a += 1,
                C => self.c += 1,
                G => self.g += 1,
                T => self.t += 1,
            }
        }
    }
}

impl std::iter::FromIterator<Nucleotide> for NucleotideCount {
    fn from_iter<I: IntoIterator<Item = Nucleotide>>(iterator: I) -> Self {
        let mut counts = Self::new();

        counts.extend(iterator);

        counts
    }
}

impl From<NucleotideCount> for HashMap<char, usize> {
    fn from(counts: NucleotideCount) -> Self {
        [
            (char::from(A), counts.a),
            (char::from(C), counts.c),
            (char::from(G), counts.g),
            (char::from(T), counts.t),
        ]
        .iter()
        .cloned()
        .collect()
    }
}
