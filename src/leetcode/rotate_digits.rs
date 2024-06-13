pub fn rotated_digits(n: i32) -> i32 {
    (1..=n).filter(good_after_rotating).count() as i32
}

fn good_after_rotating(number: &i32) -> bool {
    let mut digits = vec![];
    let mut value = *number;

    while value > 0 {
        digits.push(value % 10);
        value /= 10;
    }

    let mut rotated = 0;
    let mut offset = 1;
    for digit in digits.iter() {
        let rotated_digit = match digit {
            0 | 1 | 8 => *digit,
            5 => 2,
            2 => 5,
            6 => 9,
            9 => 6,
            _ => return false,
        };
        rotated += rotated_digit * offset;
        offset *= 10;
    }

    rotated != *number
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
