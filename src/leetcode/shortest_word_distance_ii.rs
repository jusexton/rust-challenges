use std::collections::HashMap;

struct WordDistance {
    positions: HashMap<String, Vec<usize>>,
}

impl WordDistance {
    fn new(word_dict: Vec<String>) -> Self {
        let positions =
            word_dict
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (idx, word)| {
                    let values: &mut Vec<_> = acc.entry(word).or_default();
                    values.push(idx);
                    acc
                });
        Self { positions }
    }

    fn shortest(&self, word1: String, word2: String) -> i32 {
        let word1_indexes = self.positions.get(&word1).unwrap();
        let word2_indexes = self.positions.get(&word2).unwrap();

        let mut shortest = i32::MAX;
        let (mut i, mut j) = (0, 0);
        while i < word1_indexes.len() && j < word2_indexes.len() {
            let word1_idx = word1_indexes[i];
            let word2_idx = word2_indexes[j];
            shortest = match word1_idx < word2_idx {
                true => {
                    i += 1;
                    shortest.min((word2_idx - word1_idx) as i32)
                }
                false => {
                    j += 1;
                    shortest.min((word1_idx - word2_idx) as i32)
                }
            };
        }
        shortest
    }
}

#[cfg(test)]
mod tests {
    use crate::string_vec;

    use super::*;

    #[test]
    fn finds_shortest_distance_between_words() {
        let word_dict = string_vec!["practice", "makes", "perfect", "coding", "makes"];
        let word_distance = WordDistance::new(word_dict);
        assert_eq!(
            3,
            word_distance.shortest("coding".to_string(), "practice".to_string())
        );
    }

    #[test]
    fn finds_shortest_distance_when_equal_words_are_closest() {
        let word_dict = string_vec!["a", "b", "c", "d", "d"];
        let word_distance = WordDistance::new(word_dict);
        assert_eq!(3, word_distance.shortest("a".to_string(), "d".to_string()));
    }
}
