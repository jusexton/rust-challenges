fn rotate(numbers: &mut [i32], k: i32) {
    let k = k as usize % numbers.len();
    numbers.rotate_right(k)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::rotate;

    #[test_case(&mut [-1], 2, &[-1])]
    #[test_case(&mut [1, 2, 3], 1, &[3, 1, 2])]
    #[test_case(&mut [1, 2], 3, &[2, 1])]
    fn test_rotate(numbers: &mut [i32], k: i32, expected: &[i32]) {
        rotate(numbers, k);
        assert_eq!(numbers, expected)
    }
}
