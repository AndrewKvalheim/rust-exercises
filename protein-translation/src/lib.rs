mod codons_info;
mod translation;
mod utilities;

use codons_info::CodonsInfo;

pub fn parse<'a, T: Into<CodonsInfo<'a>>>(rules: T) -> CodonsInfo<'a> {
    rules.into()
}
