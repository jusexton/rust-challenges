// O(nm) Solution
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&n| n < 0)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_negative_values() {
        assert_eq!(
            8,
            count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ])
        )
    }
}
