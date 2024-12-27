pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut placements = vec![vec![false; n]; m];
    for row in 0..m {
        for col in 0..n {
            if matrix[row][col] == 0 && !placements[row][col] {
                for i in 0..n {
                    if matrix[row][i] != 0 {
                        matrix[row][i] = 0;
                        placements[row][i] = true;
                    }
                }
                for i in 0..m {
                    if matrix[i][col] != 0 {
                        matrix[i][col] = 0;
                        placements[i][col] = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_matrix_rows_and_columns_to_zeroes_if_zero_is_in_that_row_or_column() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix);
        assert_eq!(
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
            matrix
        )
    }
}
