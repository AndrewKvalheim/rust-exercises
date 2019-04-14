mod dna;
mod rna;
mod sequence;

use sequence::Sequence;

pub type DNA = Sequence<dna::Base>;
pub type RNA = Sequence<rna::Base>;

impl DNA {
    pub fn into_rna(self) -> RNA {
        self.to_base::<rna::Base>()
    }
}
