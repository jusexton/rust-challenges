use std::cmp;
use std::collections::HashMap;

fn max_length_between_equal_characters(s: String) -> i32 {
    let mut occurrences = HashMap::new();
    let mut max_length = -1;
    for (i, c) in s.char_indices() {
        if let Some(value) = occurrences.get(&c) {
            max_length = cmp::max(max_length, (i - value - 1) as i32);
        } else {
            occurrences.insert(c, i);
        }
    }

    max_length
}

#[cfg(test)]
mod tests {
    use crate::leetcode::max_substr_between_duplicates::max_length_between_equal_characters;
    use test_case::test_case;

    #[test_case("aa", 0)]
    #[test_case("abba", 2)]
    fn should_produce_length_of_largest_substring_between_duplicates(s: &str, expected: i32) {
        let actual = max_length_between_equal_characters(s.to_string());
        assert_eq!(actual, expected);
    }
}
