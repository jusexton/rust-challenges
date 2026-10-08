pub fn remove_outer_parentheses(s: String) -> String {
    let mut result = String::with_capacity(s.len());
    let mut depth = 0;
    for ch in s.chars() {
        match ch {
            '(' => {
                if depth > 0 {
                    result.push(ch);
                }
                depth += 1;
            }
            ')' => {
                depth -= 1;
                if depth > 0 {
                    result.push(ch);
                }
            }
            _ => {}
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_outer_parentheses() {
        assert_eq!(
            "()()()".to_string(),
            remove_outer_parentheses("(()())(())".to_string())
        );
        assert_eq!(
            "()()()()(())".to_string(),
            remove_outer_parentheses("(()())(())(()(()))".to_string())
        );
    }
}
