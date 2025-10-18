fn type_str(s: String) -> String {
    s.chars().fold(String::new(), |mut acc, curr| {
        if curr == '#' {
            acc.pop();
        } else {
            acc.push(curr);
        }
        acc
    })
}

pub fn backspace_compare(s: String, t: String) -> bool {
    type_str(s) == type_str(t)
}

#[cfg(test)]
mod tests {
    use super::backspace_compare;

    #[test]
    fn equal_with_no_backspaces() {
        assert!(backspace_compare("abc".to_string(), "abc".to_string()))
    }

    #[test]
    fn equal_with_backspaces() {
        assert!(backspace_compare("abc##".to_string(), "ab#".to_string()))
    }

    #[test]
    fn not_equal_with_no_backspaces() {
        assert!(!backspace_compare("a".to_string(), "abc".to_string()))
    }

    #[test]
    fn not_equal_with_backspaces() {
        assert!(!backspace_compare(
            "abc##".to_string(),
            "ab#####".to_string()
        ))
    }

    #[test]
    fn only_backspace() {
        assert!(backspace_compare("##".to_string(), "####".to_string()))
    }
}
