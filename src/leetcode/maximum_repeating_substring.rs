pub fn max_repeating(sequence: String, word: String) -> i32 {
    let mut sub_str = "".to_string();
    while sequence.contains(&sub_str) {
        sub_str.push_str(&word)
    }
    let result = (sub_str.len() - word.len()) / word.len();
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_repeating() {
        let actual = max_repeating("ababc".to_string(), "ab".to_string());
        assert_eq!(2, actual);

        let actual = max_repeating("ababc".to_string(), "abcd".to_string());
        assert_eq!(0, actual)
    }
}
