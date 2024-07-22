const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

pub fn halves_are_alike(s: String) -> bool {
    fn count_vowels(s: &str) -> usize {
        s.chars().filter(|c| VOWELS.contains(c)).count()
    }
    let middle = s.len() / 2;
    count_vowels(&s[0..middle]) == count_vowels(&s[middle..])
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
