// https://www.codewars.com/kata/52774a314c2333f0a7000688
fn valid_parentheses(s: &str) -> bool {
    let mut open_count = 0;
    for char in s.chars() {
        match char {
            '(' => open_count += 1,
            ')' if open_count > 0 => open_count -= 1,
            ')' => return false,
            _ => ()
        }
    }
    open_count == 0
}

#[cfg(test)]
mod tests {
    use crate::codewars::valid_parentheses::valid_parentheses;

    #[test]
    fn test_parentheses_are_validated_correctly() {
        let test_cases = vec![
            ("", true),
            ("()", true),
            ("()()", true),
            ("())", false),
            (")(", false)
        ];

        for (parentheses, expected) in test_cases {
            let actual = valid_parentheses(parentheses);
            assert_eq!(expected, actual);
        }
    }
}
