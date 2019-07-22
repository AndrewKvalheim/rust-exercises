mod digit_scan;
mod iter_groups;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidColumnCount(usize),
    InvalidRowCount(usize),
}

pub fn convert(image: &str) -> Result<String, Error> {
    iter_groups::iter_groups(image.lines())
        .collect::<Result<Vec<_>, _>>()
        .map(|g| g.join(","))
}
