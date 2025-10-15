pub fn score_of_a_string(s: String) -> i32 {
    s.as_bytes()
        .windows(2)
        .map(|characters| ((characters[0] as i32) - (characters[1] as i32)).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::score_of_a_string::score_of_a_string;
    use test_case::test_case;

    #[test_case("hello", 13)]
    #[test_case("zaz", 50)]
    fn test_score_of_string(s: &str, expected: i32) {
        assert_eq!(score_of_a_string(s.to_owned()), expected);
    }
}
