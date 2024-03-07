// https://www.codewars.com/kata/5264d2b162488dc400000001
fn spinning_words(words: &str) -> String {
    words
        .split_ascii_whitespace()
        .map(|word| match word.len() >= 5 {
            true => word.chars().rev().collect(),
            false => word.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::spinning_words::spinning_words;

    #[test_case("", "")]
    #[test_case("welcome", "emoclew")]
    #[test_case("Hey fellow warriors", "Hey wollef sroirraw")]
    fn test_spinning_words(input: &str, expected: &str) {
        assert_eq!(expected, spinning_words(input))
    }
}
