pub fn number_of_special_chars(word: String) -> i32 {
    let mut found = [0; 26];
    for ch in word.bytes() {
        if ch.is_ascii_lowercase() {
            let i = (ch - b'a') as usize;
            found[i] |= ((found[i] & 0b10) << 1) | 0b01;
        } else {
            found[(ch - b'A') as usize] |= 0b10;
        }
    }
    found.into_iter().filter(|&x| x == 0b11).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::leetcode::count_the_number_of_special_characters_ii::number_of_special_chars;

    #[test]
    fn counts_special_characters() {
        assert_eq!(3, number_of_special_chars("aaAbcBC".to_string()));
        assert_eq!(0, number_of_special_chars("abc".to_string()));
        assert_eq!(0, number_of_special_chars("AbBCab".to_string()));
    }
}
