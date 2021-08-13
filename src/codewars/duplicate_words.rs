use std::collections::HashSet;

fn remove_duplicate_words(s: &str) -> String {
    let mut result = vec![];
    let mut seen = HashSet::new();

    for word in s.split_whitespace() {
        if seen.insert(word) {
            result.push(word);
        }
    }

    result.join(" ")
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::duplicate_words::remove_duplicate_words;

    #[test_case("test test", "test")]
    #[test_case("test test cat", "test cat")]
    fn should_remove_duplicate_words_from_string(s: &str, expected: &str) {
        let actual = remove_duplicate_words(s);
        assert_eq!(actual, expected.to_string());
    }
}
