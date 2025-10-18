pub fn find_numbers(numbers: Vec<i32>) -> i32 {
    numbers.into_iter().fold(0, |acc, mut curr| {
        let mut digit_count = 0;
        while curr > 0 {
            digit_count += 1;
            curr /= 10;
        }
        if digit_count % 2 == 0 { acc + 1 } else { acc }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zero_when_no_number_with_even_count_of_digits() {
        assert_eq!(0, find_numbers(vec![123, 123]));
    }

    #[test]
    fn zero_when_no_numbers() {
        assert_eq!(0, find_numbers(vec![]));
    }

    #[test]
    fn correct_count_of_numbers_with_even_count_of_digits() {
        assert_eq!(2, find_numbers(vec![12, 345, 2, 6, 7896]));
    }
}
