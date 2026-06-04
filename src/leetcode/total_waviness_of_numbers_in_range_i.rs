pub fn total_waviness(first: i32, last: i32) -> i32 {
    fn get_digits(mut number: i32) -> Vec<i32> {
        let mut digits = Vec::new();
        while number > 0 {
            digits.push(number % 10);
            number /= 10;
        }
        digits
    }

    fn count_waviness(number: i32) -> i32 {
        let digits = get_digits(number);
        let mut count = 0;
        for window in digits.windows(3) {
            let [left, curr, right] = window else {
                panic!("should never happen");
            };
            if curr > left && curr > right || curr < left && curr < right {
                count += 1;
            }
        }

        count
    }

    (first..=last).map(count_waviness).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_waviness() {
        assert_eq!(3, total_waviness(120, 130));
        assert_eq!(3, total_waviness(198, 202));
        assert_eq!(2, total_waviness(4848, 4848));
    }
}
