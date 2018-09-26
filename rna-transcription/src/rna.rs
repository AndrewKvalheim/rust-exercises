use crate::dna;
use crate::rfc_1542::TryFrom;
use Base::*;

#[derive(Debug, PartialEq)]
pub enum Base { A, C, G, U }

impl<'a> From<&'a dna::Base> for Base {
    fn from(base: &'a dna::Base) -> Self {
        match base {
            dna::Base::A => U,
            dna::Base::C => G,
            dna::Base::G => C,
            dna::Base::T => A,
        }
    }
}

impl TryFrom<char> for Base {
    type Error = char;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            'A' => Ok(A),
            'C' => Ok(C),
            'G' => Ok(G),
            'U' => Ok(U),
            _ => Err(character),
        }
    }
}
