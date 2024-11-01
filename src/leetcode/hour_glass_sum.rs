pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut result = i32::MIN;
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let sum = grid[i - 1][j - 1]
                + grid[i - 1][j]
                + grid[i - 1][j + 1]
                + grid[i][j]
                + grid[i + 1][j - 1]
                + grid[i + 1][j]
                + grid[i + 1][j + 1];
            result = result.max(sum);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn single_hourglass() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(35, max_sum(grid));
    }

    #[test]
    fn stacked_hourglasses() {
        let grid = vec![vec![6, 2, 1], vec![4, 2, 1], vec![9, 2, 8], vec![4, 1, 2]];
        assert_eq!(30, max_sum(grid));
    }

    #[test]
    fn square_hourglasses() {
        let grid = vec![
            vec![6, 2, 1, 3],
            vec![4, 2, 1, 5],
            vec![9, 2, 8, 7],
            vec![4, 1, 2, 9],
        ];
        assert_eq!(30, max_sum(grid));
    }
}
