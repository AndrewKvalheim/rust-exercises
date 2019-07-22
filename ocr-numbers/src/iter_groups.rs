use super::Error::{self, *};
use crate::digit_scan::DigitScan;

pub struct IterGroups<'a, I: 'a + Iterator<Item = &'a str>> {
    row_count: usize,
    rows: I,
}

impl<'a, I: 'a + Iterator<Item = &'a str>> Iterator for IterGroups<'a, I> {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| {
            self.row_count += 1;

            let column_count = row.len();

            if column_count % 3 != 0 {
                return Err(InvalidColumnCount(column_count));
            }

            let mut digit_scans = row
                .as_bytes()
                .chunks(3)
                .map(DigitScan::from)
                .collect::<Vec<_>>();

            for _ in 1..4 {
                let row = self
                    .rows
                    .next()
                    .ok_or_else(|| InvalidRowCount(self.row_count))?;

                self.row_count += 1;

                if row.len() != column_count {
                    return Err(InvalidColumnCount(row.len()));
                }

                digit_scans
                    .iter_mut()
                    .zip(row.as_bytes().chunks(3))
                    .for_each(|(node, chunk)| node.step(chunk));
            }

            Ok(digit_scans.iter().map(char::from).collect())
        })
    }
}

pub fn iter_groups<'a, I: 'a + Iterator<Item = &'a str>>(rows: I) -> IterGroups<'a, I> {
    IterGroups { row_count: 0, rows }
}
