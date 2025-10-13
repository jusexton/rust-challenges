pub fn min_changes(n: i32, k: i32) -> i32 {
    let mut changes = 0;
    // The constraint is
    // 1 <= n, k <= 10^6
    // So bit 20 is the highest bit we might change
    for i in 0..=20 {
        match (k & (1 << i) != 0, n & (1 << i) != 0) {
            (false, false) | (true, true) => {}
            (false, true) => {
                changes += 1;
            }
            (true, false) => {
                return -1;
            }
        }
    }
    changes
}

#[cfg(test)]
mod tests {
    use super::min_changes;
    use test_case::test_case;

    #[test_case(13, 4, 2)]
    #[test_case(21, 21, 0)]
    #[test_case(14, 13, -1)]
    fn test_min_changes(n: i32, k: i32, expected: i32) {
        assert_eq!(min_changes(n, k), expected);
    }
}
