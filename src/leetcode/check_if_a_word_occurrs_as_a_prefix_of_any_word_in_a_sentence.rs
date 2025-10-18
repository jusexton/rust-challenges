pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    sentence
        .split_ascii_whitespace()
        .position(|word| word.starts_with(&search_word))
        .map_or(-1, |idx| (idx + 1) as i32)
}

#[cfg(test)]
mod tests {
    use super::is_prefix_of_word;

    #[test]
    fn finds_index_when_word_prefixed_with_search_word() {
        assert_eq!(
            4,
            is_prefix_of_word("i love eating burger".to_string(), "burg".to_string())
        );

        assert_eq!(
            2,
            is_prefix_of_word(
                "this problem is an easy problem".to_string(),
                "pro".to_string()
            )
        )
    }

    #[test]
    fn negative_one_when_word_not_prefixed_with_search_word() {
        assert_eq!(
            -1,
            is_prefix_of_word("i am tired".to_string(), "you".to_string())
        )
    }
}
