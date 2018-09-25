use Nucleotide::*;

#[derive(PartialEq)]
pub enum Nucleotide {
    A,
    C,
    G,
    T,
}

impl Nucleotide {
    // Pending RFC 1542
    pub fn try_from(character: char) -> Result<Self, char> {
        match character {
            'A' => Ok(A),
            'C' => Ok(C),
            'G' => Ok(G),
            'T' => Ok(T),
            _ => Err(character),
        }
    }
}

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
