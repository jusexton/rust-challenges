use std::collections::HashSet;

pub fn halves_are_alike(s: String) -> bool {
    fn count_vowels(s: &str) -> usize {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        s.chars().filter(|c| vowels.contains(c)).count()
    }

    let middle = s.len() / 2;
    let first = &s.as_str()[0..middle];
    let second = &s.as_str()[middle..];

    count_vowels(first) == count_vowels(second)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::alike_halves::halves_are_alike;

    #[test_case("book", true)]
    #[test_case("textbook", false)]
    #[test_case("Uo", true)]
    fn test(s: &str, expected: bool) {
        let actual = halves_are_alike(s.to_string());
        assert_eq!(actual, expected)
    }
}
