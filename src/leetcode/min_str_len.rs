pub fn min_length(s: String) -> i32 {
    let mut stack = Vec::new();
    for ch in s.chars() {
        if stack.is_empty() {
            stack.push(ch);
            continue;
        }

        let is_ab_match = ch == 'B' && stack[stack.len() - 1] == 'A';
        let is_cd_match = ch == 'D' && stack[stack.len() - 1] == 'C';
        if !stack.is_empty() && (is_ab_match || is_cd_match) {
            stack.pop();
        } else {
            stack.push(ch);
        }
    }

    stack.len() as i32
}

#[cfg(test)]
mod tests {
    use super::min_length;

    #[test]
    fn zero_length() {
        assert_eq!(0, min_length("AAABBB".to_string()))
    }

    #[test]
    fn minimum_length() {
        assert_eq!(2, min_length("ABHRCABD".to_string()))
    }

    #[test]
    fn original_length() {
        assert_eq!(4, min_length("ACBD".to_string()))
    }

    #[test]
    fn single_character() {
        assert_eq!(1, min_length("D".to_string()))
    }
}
