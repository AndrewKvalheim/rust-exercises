#![allow(clippy::naive_bytecount)]

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    padded_indexable(minefield)
        .windows(3)
        .map(|rows| {
            zip3(rows[0].windows(3), rows[1].windows(3), rows[2].windows(3))
                .map(|(a, b, c)| (b[1], [a[0], b[0], c[0], a[1], c[1], a[2], b[2], c[2]]))
                .map(|(cell, adjacent)| match cell {
                    b'*' => '*',
                    _ => match adjacent.iter().filter(|&&c| c == b'*').count() as u8 {
                        0 => ' ',
                        n => char::from(b'0' + n),
                    },
                })
                .collect()
        })
        .collect()
}

fn padded_indexable(minefield: &[&str]) -> Vec<Vec<u8>> {
    let width = 2 + minefield.first().map_or(0, |r| r.len());

    let mut rows = Vec::with_capacity(2 + minefield.len());

    rows.push(vec![b' '; width]);
    rows.extend(minefield.iter().map(|&line| {
        let mut row = Vec::with_capacity(width);

        row.push(b' ');
        row.extend_from_slice(line.as_bytes());
        row.push(b' ');

        row
    }));
    rows.push(vec![b' '; width]);

    rows
}

fn zip3<T>(
    a: impl Iterator<Item = T>,
    b: impl Iterator<Item = T>,
    c: impl Iterator<Item = T>,
) -> impl Iterator<Item = (T, T, T)> {
    a.zip(b).zip(c).map(|((a, b), c)| (a, b, c))
}
