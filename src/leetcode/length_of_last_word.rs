pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace()
        .last()
        .map_or(0, |word| word.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::length_of_last_word;

    #[test]
    fn empty_string() {
        assert_eq!(0, length_of_last_word("".to_string()))
    }

    #[test]
    fn without_padded_space() {
        assert_eq!(4, length_of_last_word("fly me to the moon".to_string()))
    }

    #[test]
    fn with_padded_space() {
        assert_eq!(
            4,
            length_of_last_word("   fly me   to   the   moon  ".to_string())
        )
    }
}
