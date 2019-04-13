mod nucleotide;
mod nucleotide_count;

use crate::nucleotide_count::NucleotideCount;
use nucleotide::Nucleotide;
use std::collections::HashMap;
use std::convert::TryFrom;

pub fn count(query: char, sequence: &str) -> Result<usize, char> {
    let nucleotide = Nucleotide::try_from(query)?;

    sequence
        .chars()
        .map(Nucleotide::try_from)
        .try_fold(0, |c, n| Ok(if n? == nucleotide { c + 1 } else { c }))
}

pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, char> {
    NucleotideCount::try_from(sequence).map(HashMap::from)
}
