pub fn compressed_string(word: String) -> String {
    let mut freq = 1;
    let mut prev_char = word.chars().next().unwrap();
    let mut compressed = String::new();
    for curr_char in word.chars().skip(1) {
        match curr_char == prev_char {
            true => {
                if freq == 9 {
                    compressed.push_str(&format!("{freq}{prev_char}"));
                    freq = 0
                }
                freq += 1;
            }
            false => {
                compressed.push_str(&format!("{freq}{prev_char}"));
                prev_char = curr_char;
                freq = 1;
            }
        }
    }
    compressed.push_str(&format!("{freq}{prev_char}"));
    compressed
}

#[cfg(test)]
mod tests {
    use crate::leetcode::string_compression::compressed_string;

    #[test]
    fn one_of_each() {
        let word = "abcde".to_string();
        assert_eq!("1a1b1c1d1e".to_string(), compressed_string(word))
    }

    #[test]
    fn more_than_nine_consecutive_chars() {
        let word = "aaaaaaaaaaaaaabb".to_string();
        assert_eq!("9a5a2b".to_string(), compressed_string(word))
    }
}
