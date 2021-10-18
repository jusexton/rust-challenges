fn minimum_number(numbers: &[u32]) -> u32 {
    let total: u32 = numbers.iter().sum();
    let mut result = 0;
    while !is_prime(total + result) {
        result += 1;
    }
    result
}

fn is_prime(number: u32) -> bool {
    let limit = (number as f64).sqrt() as u32;
    !(2..=limit).any(|i| number % i == 0)
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::nearest_prime::is_prime;
    use crate::codewars::nearest_prime::minimum_number;

    #[test_case(&[3, 1, 2], 1)]
    #[test_case(&[1, 1, 1], 0)]
    #[test_case(&[3, 2, 2], 0)]
    #[test_case(&[50, 39, 49, 6, 17, 28], 2)]
    fn should_return_the_smallest_number_to_sum_to_nearest_prime(numbers: &[u32], expected: u32) {
        let actual = minimum_number(numbers);
        assert_eq!(actual, expected);
    }

    #[test_case(1, true)]
    #[test_case(2, true)]
    #[test_case(3, true)]
    #[test_case(4, false)]
    #[test_case(5, true)]
    fn should_return_whether_given_number_is_prime_or_not(number: u32, expected: bool) {
        let actual = is_prime(number);
        assert_eq!(actual, expected);
    }
}
