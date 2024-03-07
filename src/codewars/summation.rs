fn summation(number: i32) -> i32 {
    (1..=number).sum()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::summation::summation;

    #[test_case(2, 3)]
    #[test_case(8, 36)]
    fn test_summation(number: i32, expected: i32) {
        let actual = summation(number);
        assert_eq!(actual, expected)
    }
}
