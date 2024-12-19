pub fn is_palindrome(s: String) -> bool {
    let s = s.as_bytes();
    let (mut left, mut right) = (0, s.len() - 1);
    while left < right {
        if !s[left].is_ascii_alphanumeric() {
            left += 1;
        } else if !s[right].is_ascii_alphanumeric() {
            right -= 1;
        } else {
            if s[left].to_ascii_lowercase() != s[right].to_ascii_lowercase() {
                return false;
            }
            left += 1;
            right -= 1;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_if_given_string_is_palindrome() {
        assert!(is_palindrome(" ".to_string()));
        assert!(is_palindrome("a.".to_string()));
        assert!(is_palindrome("tacocat".to_string()));
        assert!(is_palindrome("t  a.co ca t".to_string()));
        assert!(is_palindrome("T  a.cO cA t".to_string()))
    }

    #[test]
    fn determines_if_given_string_is_not_palindrome() {
        assert!(!is_palindrome("not palindrome".to_string()))
    }
}
