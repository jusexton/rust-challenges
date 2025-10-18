pub fn largest_odd_number(number: String) -> String {
    for (index, c) in number.char_indices().rev() {
        if c.to_digit(10).unwrap() % 2 == 1 {
            return number[0..=index].to_string();
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::largest_odd_number_in_string::largest_odd_number;

    #[test_case("5", "52")]
    #[test_case("", "246")]
    #[test_case("35427", "35427")]
    fn test_length_of_longest_substring(expected: &str, s: &str) {
        let result = largest_odd_number(s.to_string());
        assert_eq!(expected.to_string(), result);
    }
}
