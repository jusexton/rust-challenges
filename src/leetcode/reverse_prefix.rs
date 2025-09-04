pub fn reverse_prefix(word: String, ch: char) -> String {
    match word.find(ch) {
        Some(idx) => {
            let idx = idx + 1;
            let reversed: String = word[0..idx].chars().rev().collect();
            format!("{}{}", reversed, &word[idx..])
        }
        None => word,
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::reverse_prefix::reverse_prefix;

    #[test]
    fn produces_reverse_prefix() {
        assert_eq!(
            "dcbaefd".to_string(),
            reverse_prefix("abcdefd".to_string(), 'd')
        );
    }
}
