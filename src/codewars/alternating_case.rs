fn to_alternating_case(s: &str) -> String {
    s.chars()
        .map(|character| {
            if character.is_uppercase() {
                character.to_ascii_lowercase()
            } else {
                character.to_ascii_uppercase()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::alternating_case::to_alternating_case;

    #[test_case("test", "TEST")]
    #[test_case("AB", "ab")]
    #[test_case("1234", "1234")]
    fn should_correctly_return_string_as_alternating_case(s: &str, expected: &str) {
        let actual = to_alternating_case(s);
        assert_eq!(expected, actual);
    }
}
