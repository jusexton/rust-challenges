pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let total = m * n;
    let mut result = vec![vec![0; n]; m];
    (0..m).for_each(|i| {
        for j in 0..n {
            let new_pos = (i * n + j + k as usize) % total;
            let new_column = new_pos / n;
            let new_row = new_pos % n;
            result[new_column][new_row] = grid[i][j]
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::shift_grid;

    #[test]
    fn shifts_grid_by_k() {
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        let actual = shift_grid(grid, 4);
        assert_eq!(
            vec![
                vec![12, 0, 21, 13],
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
            ],
            actual
        )
    }
}
