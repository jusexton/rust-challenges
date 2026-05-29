pub fn min_element(numbers: Vec<i32>) -> i32 {
    fn sum_digits(mut number: i32) -> i32 {
        let mut sum = 0;
        while number > 0 {
            sum += number % 10;
            number /= 10;
        }
        sum
    }

    numbers.into_iter().map(sum_digits).min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::minimum_element_after_replacement_with_digit_sum::min_element;

    #[test]
    fn should_return_min_digit_summed_element() {
        assert_eq!(1, min_element(vec![10, 12, 13, 14]));
        assert_eq!(10, min_element(vec![199, 19, 299]));
    }
}
