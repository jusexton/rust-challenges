pub fn clear_digits(s: String) -> String {
    s.chars().fold(String::new(), |mut acc, curr| {
        if curr.is_ascii_digit() {
            if !acc.is_empty() {
                acc.pop();
            }
        } else {
            acc.push(curr);
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_digits() {
        assert_eq!(clear_digits("aa11".to_string()), "".to_string());
        assert_eq!(clear_digits("abc".to_string()), "abc".to_string());
        assert_eq!(clear_digits("cb34".to_string()), "".to_string());
    }
}
