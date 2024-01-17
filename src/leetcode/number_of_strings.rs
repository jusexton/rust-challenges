use std::collections::HashSet;

pub fn number_of_different_integers(s: String) -> i32 {
    let mut numbers = HashSet::from([""]);
    let mut left = 0;
    for (right, ch) in s.char_indices() {
        if ch.is_ascii_digit() {
            if left < right && s.chars().nth(left).unwrap() == '0' {
                left += 1
            }
        } else {
            numbers.insert(&s[left..right]);
            left = right + 1;
        }
    }

    numbers.insert(&s[left..]);
    (numbers.len() - 1) as i32
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::number_of_strings::number_of_different_integers;

    #[test_case(3, "a123bc34d8ef34")]
    #[test_case(2, "leet1234code234")]
    #[test_case(1, "a1b01c001")]
    #[test_case(2, "123abc45")]
    #[test_case(0, "abc")]
    fn test_number_of_different_integers(expected: i32, input: &str) {
        let actual = number_of_different_integers(input.to_string());
        assert_eq!(expected, actual)
    }
}
