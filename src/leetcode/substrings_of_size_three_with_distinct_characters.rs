pub fn count_good_substrings(s: String) -> i32 {
    fn is_good(s: &[u8]) -> bool {
        s.len() == 3 && s[0] != s[1] && s[0] != s[2] && s[1] != s[2]
    }
    s.as_bytes().windows(3).filter(|w| is_good(w)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::count_good_substrings;

    #[test]
    fn counts_substrings_of_size_three_that_are_good() {
        assert_eq!(1, count_good_substrings("xyzzaz".to_string()))
    }
}
