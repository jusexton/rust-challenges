pub fn min_operations(s: String) -> i32 {
    let distance_from_a = |c| (26 - (c as i32 - 97)) % 26;
    s.chars().map(distance_from_a).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::min_operations;
    use test_case::test_case;

    #[test_case("yz", 2)]
    #[test_case("a", 0)]
    fn test_min_operations(s: &str, expected: i32) {
        assert_eq!(min_operations(s.into()), expected);
    }
}
