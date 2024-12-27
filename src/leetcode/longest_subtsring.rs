use std::cmp;
use std::collections::HashMap;

pub fn longest_substring(s: String) -> i32 {
    let mut result = 0;
    let mut left = 0;
    let mut char_indexes = HashMap::new();

    for (right, char) in s.char_indices() {
        if let Some(&index) = char_indexes.get(&char) {
            left = cmp::max(left, index + 1)
        }

        result = cmp::max(result, right - left + 1);
        char_indexes.insert(char, right);
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::longest_subtsring::longest_substring;

    #[test_case(0, "")]
    #[test_case(3, "abcabcbb")]
    #[test_case(1, "bbbbb")]
    #[test_case(3, "pwwkew")]
    #[test_case(5, "tmmzuxt")]
    fn test_length_of_longest_substring(expected: i32, s: &str) {
        let result = longest_substring(s.to_string());
        assert_eq!(expected, result);
    }
}
