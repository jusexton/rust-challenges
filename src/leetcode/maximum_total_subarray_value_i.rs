pub fn max_total_value(numbers: Vec<i32>, k: i32) -> i64 {
    let (mut min, mut max) = (numbers[0], numbers[0]);
    for n in numbers {
        min = min.min(n);
        max = max.max(n);
    }
    (max - min) as i64 * k as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_total_value() {
        assert_eq!(4, max_total_value(vec![1, 3, 2], 2));
        assert_eq!(12, max_total_value(vec![4, 2, 5, 1], 3));
    }
}
