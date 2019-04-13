use std::iter;

pub fn spiral_matrix(width: usize) -> Vec<Vec<u32>> {
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .cycle()
        .take(width * 2)
        .enumerate()
        .flat_map(|(i, d)| iter::repeat(d).take((width * 2 - i) / 2))
        .enumerate()
        .fold(
            (vec![vec![u32::default(); width]; width], (-1, 0)),
            |(mut matrix, mut position), (index, direction)| {
                position = (position.0 + direction.0, position.1 + direction.1);
                matrix[position.1 as usize][position.0 as usize] = 1 + index as u32;

                (matrix, position)
            },
        )
        .0
}
