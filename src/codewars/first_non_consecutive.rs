fn first_non_consecutive(numbers: &[i32]) -> Option<i32> {
    for index in 1..numbers.len() {
        let a = *numbers.get(index - 1).unwrap();
        let b = *numbers.get(index).unwrap();

        if a != b - 1 {
            return Some(b);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::first_non_consecutive::first_non_consecutive;

    #[test_case(&[1, 3], 3)]
    #[test_case(&[1, 2, 4], 4)]
    #[test_case(&[1, 2, 3, 4, 10], 10)]
    fn should_return_first_non_consecutive_number(numbers: &[i32], expected: i32) {
        let actual = first_non_consecutive(numbers).unwrap();
        assert_eq!(expected, actual)
    }

    #[test_case(&[1])]
    #[test_case(&[1, 2, 3, 4])]
    fn should_return_none_when_all_values_are_consecutive(numbers: &[i32]) {
        let actual = first_non_consecutive(numbers);
        assert!(actual.is_none())
    }
}
