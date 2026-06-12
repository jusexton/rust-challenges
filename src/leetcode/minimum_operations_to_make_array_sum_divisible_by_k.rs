pub fn min_operations(numbers: Vec<i32>, k: i32) -> i32 {
    numbers.into_iter().sum::<i32>() % k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(4, min_operations(vec![3, 9, 7], 5));
        assert_eq!(0, min_operations(vec![3, 5, 7], 5));
    }
}
