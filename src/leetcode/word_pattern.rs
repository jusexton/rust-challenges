use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let words: Vec<_> = s.split_whitespace().map(String::from).collect();
    if pattern.len() != words.len() {
        return false;
    }

    let pattern = pattern.as_bytes();
    let mut map = HashMap::new();
    for (i, word) in words.into_iter().enumerate() {
        if map.insert(pattern[i].to_string(), i) != map.insert(word, i) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_valid_patterns() {
        assert!(word_pattern("aaa".to_string(), "dog dog dog".to_string()))
    }

    #[test]
    fn identifies_invalid_patterns() {
        assert!(!word_pattern("aaa".to_string(), "dog cat dog".to_string()));
        assert!(!word_pattern(
            "abba".to_string(),
            "dog dog dog dog".to_string()
        ));
        assert!(!word_pattern("aaa".to_string(), "aa aa aa aa".to_string()))
    }
}
