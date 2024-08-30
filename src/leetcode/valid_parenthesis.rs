pub fn check_valid_string(s: String) -> bool {
    let (mut left_min, mut left_max) = (0, 0);
    for ch in s.chars() {
        match ch {
            '(' => {
                left_min += 1;
                left_max += 1;
            }
            ')' => {
                left_min -= 1;
                left_max -= 1;
            }
            _ => {
                left_min -= 1;
                left_max += 1;
            }
        }

        if left_max < 0 {
            return false;
        }
        if left_min < 0 {
            left_min = 0;
        }
    }

    left_min == 0
}

#[cfg(test)]
mod tests {
    use crate::leetcode::valid_parenthesis::check_valid_string;

    #[test]
    fn trivial() {
        assert!(check_valid_string("()".to_string()));
        assert!(!check_valid_string(")(".to_string()))
    }

    #[test]
    fn empty_string_wildcard() {
        assert!(check_valid_string("(*)".to_string()));
    }

    #[test]
    fn wildcard_completes_closing_parenthesis() {
        assert!(check_valid_string("((*)".to_string()));
    }

    #[test]
    fn wildcard_completes_opening_parenthesis() {
        assert!(check_valid_string("(*))".to_string()));
    }
}
