fn nth_power(numbers: &[u64], index: usize) -> Option<u64> {
    numbers.get(index).map(|number| number.pow(index as u32))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::nth_power::nth_power;

    #[test_case(&[1, 2, 3], 0, Some(1))]
    #[test_case(&[1, 2, 3], 1, Some(2))]
    #[test_case(&[1, 2, 3], 2, Some(9))]
    fn should_return_nth_power_of_nth_index(numbers: &[u64], index: usize, expected: Option<u64>) {
        let result = nth_power(numbers, index);
        assert_eq!(result, expected)
    }

    #[test]
    fn should_return_none_when_given_index_is_out_of_range() {
        let result = nth_power(&[1, 2, 3], 5);
        assert_eq!(result, None)
    }
}
