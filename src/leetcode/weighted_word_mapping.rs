pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
    words
        .into_iter()
        .map(|word| {
            let sum: i32 = word
                .chars()
                .map(|c| weights[c as usize - 'a' as usize])
                .sum();
            (122 - (sum % 26) as u8) as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::string_vec;

    use super::*;

    #[test]
    fn test_map_word_weights() {
        assert_eq!(
            "rij".to_string(),
            map_word_weights(
                string_vec!["abcd", "def", "xyz"],
                vec![
                    5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7,
                    2
                ]
            )
        );
        assert_eq!(
            "yyy".to_string(),
            map_word_weights(
                string_vec!["a", "b", "c"],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                ]
            )
        );
        assert_eq!(
            "cmkcmj".to_string(),
            map_word_weights(
                string_vec!["a", "ehixqfz", "rorew", "cobaelzi", "ietptx", "nhp"],
                vec![
                    49, 46, 44, 12, 34, 11, 32, 13, 23, 22, 15, 37, 24, 14, 23, 15, 20, 32, 14, 28,
                    24, 35, 50, 41, 22, 27
                ]
            )
        )
    }
}
