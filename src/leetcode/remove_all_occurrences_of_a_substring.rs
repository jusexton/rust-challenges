fn remove_occurrences(s: String, part: String) -> String {
    let mut result = s;
    while result.contains(&part) {
        result = result.replacen(&part, "", 1)
    }
    result
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::remove_all_occurrences_of_a_substring::remove_occurrences;

    #[test_case("abc", "xy", "abc")]
    #[test_case("axxyyb", "xy", "ab")]
    #[test_case("aabababa", "aba", "ba")]
    fn test_remove_occurrences(s: &str, part: &str, expected: &str) {
        let actual = remove_occurrences(s.to_string(), part.to_string());
        assert_eq!(actual, expected.to_string());
    }
}
