use std::collections::HashSet;

fn get_digit_square_sum(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10).pow(2);
        n /= 10;
    }
    sum
}

pub fn is_happy(n: i32) -> bool {
    let mut seen = HashSet::new();
    let mut curr = get_digit_square_sum(n as u64);
    while seen.insert(curr) {
        if curr == 1 {
            return true;
        } 
        curr = get_digit_square_sum(curr);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_happy_numbers() {
        assert!(is_happy(19))
    }

    #[test]
    fn identifies_unhappy_numbers() {
        assert!(!is_happy(2))
    }

    #[test]
    fn calculates_sum_square_of_digits() {
        assert_eq!(82, get_digit_square_sum(19));
        assert_eq!(68, get_digit_square_sum(82));
        assert_eq!(100, get_digit_square_sum(68));
        assert_eq!(1, get_digit_square_sum(100))
    }
}
