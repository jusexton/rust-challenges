use std::collections::HashMap;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let s1_words = s1.split_ascii_whitespace();
    let s2_words = s2.split_ascii_whitespace();
    s1_words
        .chain(s2_words)
        .fold(HashMap::new(), |mut acc, curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|entry| entry.0.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::uncommon_from_sentences;

    #[test]
    fn single_uncommon_per_sentence() {
        let result = uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        );
        assert_vec_any_order(vec!["sweet".to_string(), "sour".to_string()], result)
    }

    #[test]
    fn single_uncommon_in_one_sentence() {
        let result = uncommon_from_sentences("apple apple".to_string(), "banana".to_string());
        assert_vec_any_order(vec!["banana".to_string()], result)
    }

    #[test]
    fn no_uncommon_in_any_sentence() {
        let result =
            uncommon_from_sentences("apple apple".to_string(), "banana banana".to_string());
        assert_vec_any_order(vec![], result)
    }

    fn assert_vec_any_order<T: Ord + std::fmt::Debug>(mut expected: Vec<T>, mut actual: Vec<T>) {
        expected.sort_unstable();
        actual.sort_unstable();
        assert_eq!(expected, actual)
    }
}
