pub fn min_remove_to_make_valid(s: String) -> String {
    let mut stack = vec![];
    let mut s = s.chars().collect::<Vec<_>>();
    for (index, ch) in s.iter_mut().enumerate() {
        match ch {
            '(' => stack.push(index),
            ')' => {
                if stack.is_empty() {
                    *ch = '*';
                } else {
                    stack.pop();
                }
            }
            _ => {}
        }
    }
    for idx in stack {
        s[idx] = '*';
    }
    s.iter().filter(|&&p| p != '*').copied().collect::<String>()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::minimum_remove_to_make_valid_parentheses::min_remove_to_make_valid;

    #[test_case("le(etco(de", "leetcode")]
    #[test_case("le(etco()de)", "le(etco()de)")]
    fn test_remove_occurrences(s: &str, expected: &str) {
        let actual = min_remove_to_make_valid(s.to_string());
        assert_eq!(actual, expected.to_string());
    }
}
