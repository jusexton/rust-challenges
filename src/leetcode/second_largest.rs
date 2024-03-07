fn second_highest(s: String) -> i32 {
    let mut largest = -1;
    let mut second_largest = -1;
    for character in s.chars() {
        if character.is_ascii_digit() {
            let digit = character.to_digit(10).unwrap() as i32;
            if digit > largest {
                second_largest = largest;
                largest = digit;
            } else if digit > second_largest && digit != largest {
                second_largest = digit;
            }
        }
    }

    second_largest
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::second_largest::second_highest;

    #[test_case("abc123", 2)]
    #[test_case("1490ab", 4)]
    #[test_case("sjhtz8344", 4)]
    fn should_return_the_second_largest_digit_in_a_given_string(s: &str, expected: i32) {
        let result = second_highest(s.to_string());
        assert_eq!(result, expected);
    }

    #[test_case("abc")]
    #[test_case("abc111")]
    #[test_case("111")]
    fn should_return_negative_one_if_second_largest_does_not_exist(s: &str) {
        let result = second_highest(s.to_string());
        assert_eq!(result, -1);
    }
}
