use Translation::*;

const STOP_NAME: &str = "stop codon";

pub enum Translation<'a> {
    Protein(&'a str),
    Stop,
}

impl<'a> From<&'a str> for Translation<'a> {
    fn from(name: &'a str) -> Self {
        if name == STOP_NAME {
            Stop
        } else {
            Protein(name)
        }
    }
}

impl<'a> From<&'a Translation<'a>> for &'a str {
    fn from(translation: &'a Translation<'a>) -> Self {
        match translation {
            Protein(name) => name,
            Stop => STOP_NAME,
        }
    }
}
