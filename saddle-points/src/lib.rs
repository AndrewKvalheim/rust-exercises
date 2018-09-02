pub fn find_saddle_points(rows: &[Vec<u64>]) -> Vec<(usize, usize)> {
    rows.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().max().into_iter().flat_map(move |max| {
                row.iter()
                    .enumerate()
                    .filter(move |&(j, x)| x == max && rows.iter().all(|row| row[j] >= *max))
                    .map(move |(j, _)| (i, j))
            })
        }).collect()
}
