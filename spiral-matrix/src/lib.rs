const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn spiral_matrix(width: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; width]; width];
    let mut n = 0;
    let mut position = (-1, 0);

    for (i, direction) in (1..(width * 2)).zip(DIRECTIONS.iter().cycle()) {
        for _ in 0..((1 + width * 2 - i) / 2) {
            n += 1;
            position.0 += direction.0;
            position.1 += direction.1;

            matrix[position.1 as usize][position.0 as usize] = n;
        }
    }

    matrix
}
