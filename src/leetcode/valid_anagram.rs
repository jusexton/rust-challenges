use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    fn counter(s: String) -> HashMap<char, usize> {
        s.chars().fold(HashMap::new(), |mut acc, curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        })
    }
    counter(s) == counter(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_valid_anagrams() {
        assert!(is_anagram("cat".to_string(), "tac".to_string()));
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()))
    }

    #[test]
    fn identifies_invalid_anagrams() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()))
    }
}
