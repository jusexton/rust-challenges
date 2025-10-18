pub fn row_and_maximum_ones(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![0, i32::MIN];
    (0..matrix.len()).for_each(|idx| {
        let one_count = matrix[idx].iter().filter(|&&i| i == 1).count() as i32;
        if one_count > result[1] {
            result[0] = idx as i32;
            result[1] = one_count;
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_row_with_maximum_ones() {
        assert_eq!(
            vec![0, 1],
            row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]])
        );

        assert_eq!(
            vec![1, 2],
            row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]])
        );
    }
}
