use std::convert::TryFrom;
use Nucleotide::*;

#[derive(PartialEq)]
pub enum Nucleotide { A, C, G, T }

impl From<Nucleotide> for char {
    fn from(nucleotide: Nucleotide) -> char {
        match nucleotide {
            A => 'A',
            C => 'C',
            G => 'G',
            T => 'T',
        }
    }
}

impl TryFrom<char> for Nucleotide {
    type Error = char;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            'A' => Ok(A),
            'C' => Ok(C),
            'G' => Ok(G),
            'T' => Ok(T),
            _ => Err(character),
        }
    }
}
