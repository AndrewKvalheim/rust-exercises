pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut rows: Vec<Vec<u8>> = minefield.iter().map(|&r| r.as_bytes().to_vec()).collect();

    let (m, n) = (rows.first().map_or(0, |r| r.len()), rows.len());

    #[allow(clippy::needless_range_loop)] // rust-lang/rust-clippy#2277
    for j in 0..n {
        for i in 0..m {
            if rows[j][i] == b'*' {
                spread(|y| spread(|x| increment(&mut rows[y][x]), m, i), n, j);
            }
        }
    }

    rows.iter()
        .map(|r| String::from_utf8_lossy(r).into_owned())
        .collect()
}

fn increment(cell: &mut u8) {
    *cell = match cell {
        b' ' => b'1',
        b'1'...b'7' => *cell + 1,
        _ => *cell,
    };
}

fn spread<F: FnMut(usize) -> ()>(mut f: F, length: usize, index: usize) {
    if index >= 1 {
        f(index - 1);
    }

    f(index);

    if index + 1 < length {
        f(index + 1);
    }
}
