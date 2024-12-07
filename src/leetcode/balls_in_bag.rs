pub fn minimum_size(numbers: Vec<i32>, max_operations: i32) -> i32 {
    let (mut left, mut right) = (1, 1_000_000_000);
    while left < right {
        let mid = (left + right) / 2;
        let count: i32 = numbers.iter().map(|n| (n - 1) / mid).sum();
        if count > max_operations {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::minimum_size;

    #[test]
    fn finds_minimum_penalty_with_max_operations() {
        assert_eq!(3, minimum_size(vec![9], 2));
        assert_eq!(2, minimum_size(vec![2, 4, 8, 2], 4))
    }
}
