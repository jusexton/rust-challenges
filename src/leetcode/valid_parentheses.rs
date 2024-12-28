pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            _ => {
                let Some(v) = stack.pop() else {
                    return false;
                };
                if (c == ')' && v != '(') || (c == '}' && v != '{') || (c == ']' && v != '[') {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_that_parentheses_are_valid() {
        assert!(is_valid("()[]{}([{}])".to_string()))
    }

    #[test]
    fn identifies_that_parentheses_are_invalid() {
        assert!(!is_valid("()[()]{}([{}]))".to_string()))
    }
}
