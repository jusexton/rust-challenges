fn valid_spacing(s: &str) -> bool {
    if s.is_empty() {
        true
    } else {
        !s.split(' ').any(|m| m.is_empty())
    }
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::valid_spacing::valid_spacing;

    #[test_case(true, ""; "should be true when string is empty")]
    #[test_case(true, "Helloworld"; "should be true when string contains no whitespace")]
    #[test_case(true, "Hello world"; "should be true when string whitespace is valid")]
    #[test_case(false, " "; "should be false when string only contains whitespace")]
    #[test_case(false, " Hello world"; "should be false when string starts with whitespace")]
    #[test_case(false, "Hello world "; "should be false when string ends with whitespace")]
    #[test_case(false, "Hello  world"; "should be false when double whitespace is present")]
    fn should_correctly_validate_spacing(expected: bool, s: &str) {
        let result = valid_spacing(s);
        assert_eq!(expected, result);
    }
}
