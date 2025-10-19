pub fn are_numbers_ascending(s: String) -> bool {
    let tokens = s.split_ascii_whitespace();
    let mut last = i32::MIN;
    for token in tokens {
        if let Ok(number) = token.parse::<i32>() {
            if number > last {
                last = number;
            } else {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::leetcode::check_if_numbers_are_ascending_in_a_sentence::are_numbers_ascending;

    #[test]
    fn ascending_numbers() {
        assert!(are_numbers_ascending(
            "yes 1 they are 2 ascending 4".to_string()
        ))
    }

    #[test]
    fn non_ascending_numbers() {
        assert!(!are_numbers_ascending(
            "no 1 they are 2 not 2 ascending 1".to_string()
        ))
    }
}
