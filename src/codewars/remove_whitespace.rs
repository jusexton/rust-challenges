fn no_space(s: String) -> String {
    s.replace(" ", "")
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::remove_whitespace::no_space;

    #[test_case("a b c", "abc")]
    fn should_return_new_string_with_no_whitespace(s: &str, expected: &str) {
        let actual = no_space(s.to_string());
        assert_eq!(actual, expected.to_string());
    }
}
