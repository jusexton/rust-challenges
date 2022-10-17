fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::hamming_distance::hamming_distance;

    #[test_case(1, 4, 2)]
    #[test_case(1, 3, 1)]
    fn should_return_hamming_distance_between_two_numbers(
        number_one: i32,
        number_two: i32,
        expected: i32,
    ) {
        let result = hamming_distance(number_one, number_two);
        assert_eq!(result, expected);
    }
}
