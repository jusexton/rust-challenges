pub fn rotate(matrix: &mut [Vec<i32>]) {
    matrix.reverse();
    for i in 1..matrix.len() {
        for j in 0..i {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_2d_matrix_90_degrees_clockwise() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut matrix);
        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], matrix)
    }
}
