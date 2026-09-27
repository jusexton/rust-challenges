pub fn reverse_parentheses(s: String) -> String {
    let mut stack = Vec::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'(' | b'a'..=b'z' => stack.push(b),
            b')' => {
                let start = stack.iter().rposition(|&c| c == b'(').unwrap();
                stack[start + 1..].reverse();
                stack.remove(start);
            }
            _ => {}
        }
    }
    String::from_utf8(stack).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_parentheses() {
        assert_eq!("dcba", reverse_parentheses("(abcd)".to_string()));
        assert_eq!("iloveu", reverse_parentheses("(u(love)i)".to_string()));
        assert_eq!(
            "leetcode",
            reverse_parentheses("(ed(et(oc))el)".to_string())
        );
    }
}
