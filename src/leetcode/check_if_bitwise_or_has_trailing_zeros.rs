pub fn has_trailing_zeros(numbers: Vec<i32>) -> bool {
    numbers.into_iter().filter(|n| n & 1 == 0).take(2).count() == 2
}

#[cfg(test)]
mod tests {
    use super::has_trailing_zeros;

    #[test]
    fn or_trailing_zero() {
        assert!(has_trailing_zeros(vec![2, 4, 8, 16]))
    }

    #[test]
    fn no_or_trailing_zero() {
        assert!(!has_trailing_zeros(vec![1, 3, 5, 7, 9]))
    }
}
