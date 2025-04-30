struct Entry {
    idx: usize,
    word: String,
}

pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
    let positions: Vec<_> = words_dict
        .into_iter()
        .enumerate()
        .filter_map(|(idx, word)| (word == word1 || word == word2).then_some(Entry { idx, word }))
        .collect();

    let mut shortest = i32::MAX;
    for w in positions.windows(2).filter(|w| w[0].word != w[1].word) {
        let diff = w[0].idx.abs_diff(w[1].idx);
        shortest = shortest.min(diff as i32);
    }
    shortest
}

#[cfg(test)]
mod tests {
    use crate::string_vec;

    use super::*;

    #[test]
    fn finds_shortest_distance_between_words() {
        assert_eq!(
            3,
            shortest_distance(
                string_vec!["practice", "makes", "perfect", "coding", "makes"],
                "coding".to_string(),
                "practice".to_string()
            )
        );
    }

    #[test]
    fn finds_shortest_distance_when_equal_words_are_closest() {
        assert_eq!(
            3,
            shortest_distance(
                string_vec!["a", "b", "c", "d", "d"],
                "a".to_string(),
                "d".to_string()
            )
        );
    }
}
