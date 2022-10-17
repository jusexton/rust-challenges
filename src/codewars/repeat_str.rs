fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::repeat_str::repeat_str;

    #[test_case("a", 5, "aaaaa")]
    #[test_case("abc", 5, "abcabcabcabcabc")]
    fn should_return_str_repeated_x_number_of_times(src: &str, count: usize, expected: &str) {
        let actual = repeat_str(src, count);
        assert_eq!(expected.to_string(), actual);
    }
}
