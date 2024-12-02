use std::collections::HashMap;

pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    let mut result = 0;
    let mut word_freq = HashMap::new();
    for word in words.into_iter() {
        let reversed: String = word.chars().rev().collect();
        if let Some(freq) = word_freq.get(&reversed) {
            result += freq;
        }
        *word_freq.entry(word).or_insert(0i32) += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string_vec;

    #[test]
    fn counts_number_of_reverse_pairs() {
        assert_eq!(
            2,
            maximum_number_of_string_pairs(string_vec!["cd", "ac", "dc", "ca", "zz"])
        )
    }
}
