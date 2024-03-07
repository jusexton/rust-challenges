// Regex solution
// use regex::Regex;
//
// fn alphanumeric(password: &str) -> bool {
//     let regex = Regex::new("^[a-zA-Z0-9]+$").unwrap();
//     regex.is_match(password)
// }

fn alphanumeric(password: &str) -> bool {
    !password.is_empty() && password.chars().all(char::is_alphanumeric)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::not_secure::alphanumeric;

    #[test]
    fn should_return_false_when_value_is_empty() {
        let actual = alphanumeric("");
        assert!(!actual);
    }

    #[test_case("hello", true)]
    #[test_case("PassW0rd", true)]
    fn should_return_true_when_value_is_alphanumeric(value: &str, expected: bool) {
        let actual = alphanumeric(value);
        assert_eq!(actual, expected);
    }

    #[test_case("hello world_", false)]
    #[test_case("   ", false)]
    fn should_return_false_when_value_is_not_alphanumeric(value: &str, expected: bool) {
        let actual = alphanumeric(value);
        assert_eq!(actual, expected);
    }
}
