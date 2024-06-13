pub fn rotated_digits(n: i32) -> i32 {
    (1..=n).filter(good_after_rotating).count() as i32
}

fn good_after_rotating(number: &i32) -> bool {
    let mut number = *number;
    let mut valid = false;
    while number > 0 {
        let digit = number % 10;
        match digit {
            2 | 5 | 6 | 9 => valid = true,
            3 | 4 | 7 => return false,
            _ => {}
        }
        number /= 10;
    }
    valid
}

#[cfg(test)]
mod tests {
    use crate::leetcode::rotate_digits::rotated_digits;

    #[test]
    fn test_rotated_digits() {
        let result = rotated_digits(10);
        assert_eq!(result, 4);

        let result = rotated_digits(2);
        assert_eq!(result, 1);
    }
}
