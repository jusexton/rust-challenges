fn automorphic(n: u64) -> String {
    return match is_automorphic(n) {
        true => String::from("Automorphic"),
        false => String::from("Not!!"),
    };
}

fn is_automorphic(n: u64) -> bool {
    return n.pow(2).to_string().ends_with(&n.to_string());
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::automorphic::automorphic;
    use crate::codewars::automorphic::is_automorphic;

    #[test_case(1, "Automorphic")]
    #[test_case(3, "Not!!")]
    #[test_case(6, "Automorphic")]
    #[test_case(9, "Not!!")]
    #[test_case(25, "Automorphic")]
    #[test_case(53, "Not!!")]
    #[test_case(76, "Automorphic")]
    #[test_case(95, "Not!!")]
    #[test_case(625, "Automorphic")]
    #[test_case(225, "Not!!")]
    fn should_return_correct_string_based_on_given_value(value: u64, expected: &str) {
        let result = automorphic(value);
        assert_eq!(expected, result)
    }

    #[test_case(1)]
    #[test_case(6)]
    #[test_case(25)]
    #[test_case(76)]
    #[test_case(625)]
    fn should_return_true_when_given_automorphic_value(value: u64) {
        let result = is_automorphic(value);
        assert!(result)
    }

    #[test_case(3)]
    #[test_case(9)]
    #[test_case(53)]
    #[test_case(95)]
    #[test_case(225)]
    fn should_return_false_when_given_non_automorphic_value(value: u64) {
        let result = is_automorphic(value);
        assert!(!result)
    }
}
