pub fn letter_combinations(digits: String) -> Vec<String> {
    fn inner(
        digits: &str,
        output: &mut String,
        idx: usize,
        result: &mut Vec<String>,
        mappings: &[&str; 10],
    ) {
        if output.len() == digits.len() {
            result.push(output.clone());
            return;
        }

        let digit = (digits.as_bytes()[idx] - b'0') as usize;
        let characters = mappings[digit];

        for c in characters.chars() {
            output.push(c);
            inner(digits, output, idx + 1, result, mappings);
            output.pop();
        }
    }

    if digits.is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut output = String::new();
    let mappings = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    inner(&digits, &mut output, 0, &mut result, &mappings);
    result
}

#[cfg(test)]
mod tests {
    use crate::leetcode::letter_combinations::letter_combinations;

    #[test]
    fn produces_expected_characters() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            letter_combinations("23".to_string())
        )
    }

    #[test]
    fn empty_string() {
        let expected: Vec<String> = vec![];
        assert_eq!(expected, letter_combinations("".to_string()))
    }
}
