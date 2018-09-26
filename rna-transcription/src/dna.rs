use crate::rfc_1542::TryFrom;
use Base::*;

#[derive(Debug, PartialEq)]
pub enum Base { A, C, G, T }

impl TryFrom<char> for Base {
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
