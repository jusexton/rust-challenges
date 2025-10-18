pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut minimum_counts = [usize::MAX; 26];

    for word in words {
        let mut char_count = [0; 26];
        for ch in word.chars() {
            char_count[ch as usize - 'a' as usize] += 1;
        }
        for i in 0..26 {
            minimum_counts[i] = minimum_counts[i].min(char_count[i]);
        }
    }

    let mut result = Vec::new();
    (0..26).for_each(|i| {
        for _ in 0..minimum_counts[i] {
            result.push(((i as u8 + b'a') as char).to_string());
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::string_vec;

    use super::common_chars;

    #[test]
    fn returns_common_characters() {
        let words = string_vec!["bella", "roller"];
        assert_eq!(string_vec!["e", "l", "l"], common_chars(words))
    }
}
