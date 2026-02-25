pub fn find_words_containing(words: Vec<String>, c: char) -> Vec<i32> {
    words
        .into_iter()
        .enumerate()
        .filter_map(|(idx, word)| word.contains(c).then_some(idx as i32))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{leetcode::find_words_containing_character::find_words_containing, string_vec};

    #[test]
    fn test_find_words_containing() {
        assert_eq!(
            vec![0, 1],
            find_words_containing(string_vec!["leet", "code"], 'e')
        )
    }
}
