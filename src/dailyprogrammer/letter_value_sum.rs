fn letter_value_sum(s: &str) -> usize {
    s.chars().map(|c| c as usize - 96).sum()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::dailyprogrammer::letter_value_sum::letter_value_sum;

    #[test_case("a", 1)]
    #[test_case("z", 26)]
    #[test_case("cab", 6)]
    #[test_case("excellent", 100)]
    fn test_letter_value_sum(s: &str, expected: usize) {
        let actual = letter_value_sum(s);
        assert_eq!(actual, expected)
    }
}
