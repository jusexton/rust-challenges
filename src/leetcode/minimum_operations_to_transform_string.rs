pub fn min_operations(s: String) -> i32 {
    let desired_char = 'a';
    let min_char = 'a';
    let max_char = 'z';
    let char_range_len = (max_char as i32 - min_char as i32) + 1;
    s.as_bytes()
        .iter()
        .map(|&char| {
            ((desired_char as i32 - min_char as i32) - (char as i32 - min_char as i32)
                + char_range_len)
                % char_range_len
        })
        .max()
        .unwrap()
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
