pub fn separate_digits(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .flat_map(|mut n| {
            let mut result = Vec::new();
            while n > 0 {
                result.push(n % 10);
                n /= 10;
            }
            result.into_iter().rev()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::separate_digits;

    #[test]
    fn each_digit_in_array_is_separated_and_flattened() {
        let numbers = vec![123, 45, 6789];
        let digits = separate_digits(numbers);

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], digits)
    }
}
