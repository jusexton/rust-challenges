fn pascal_case(str: &str) -> String {
    str.split_whitespace()
        .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
        .collect()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::casing::pascal_case;

    #[test_case("string", "String")]
    #[test_case("test string", "TestString")]
    fn should_correctly_convert_string_to_pascal_case(input: &str, expected: &str) {
        let actual = pascal_case(input);
        assert_eq!(actual, expected)
    }
}
