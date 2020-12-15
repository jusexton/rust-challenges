fn is_perfect_square(number: i32) -> bool {
    (number as f64).sqrt().fract() == 0.0
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::perfect_square::is_perfect_square;

    #[test_case(true, 9)]
    #[test_case(true, 25)]
    #[test_case(true, 16)]
    #[test_case(false, 14)]
    #[test_case(false, 17)]
    fn should_return_whether_given_value_is_perfect_square(expected: bool, number: i32) {
        let result = is_perfect_square(number);
        assert_eq!(expected, result);
    }
}
