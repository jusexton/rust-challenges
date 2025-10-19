fn third_max(numbers: Vec<i32>) -> i32 {
    let mut max_values: [Option<i32>; 3] = [None; 3];
    for number in numbers {
        if !max_values.contains(&Some(number)) {
            if number >= max_values[0].unwrap_or(i32::MIN) {
                max_values[2] = max_values[1];
                max_values[1] = max_values[0];
                max_values[0] = Some(number);
            } else if number >= max_values[1].unwrap_or(i32::MIN) {
                max_values[2] = max_values[1];
                max_values[1] = Some(number);
            } else if number >= max_values[2].unwrap_or(i32::MIN) {
                max_values[2] = Some(number);
            }
        }
    }

    if max_values.contains(&None) {
        max_values[0].unwrap()
    } else {
        max_values[2].unwrap()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::third_maximum_number::third_max;

    #[test_case(&[3, 3, 2, 1], 1)]
    #[test_case(&[3, 2, 1], 1)]
    #[test_case(&[2, 2, 3, 1], 1)]
    #[test_case(&[3, 2], 3)]
    #[test_case(&[1, 2], 2)]
    #[test_case(&[1, 2, -2147483648], -2147483648)]
    fn should_return_the_third_max_number(numbers: &[i32], expected: i32) {
        let result = third_max(numbers.to_vec());
        assert_eq!(result, expected);
    }
}
