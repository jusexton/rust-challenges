pub fn check_divisibility(n: i32) -> bool {
    let mut number = n;
    let (mut sum, mut product) = (0, 1);
    while number > 0 {
        let digit = number % 10;
        sum += digit;
        product *= digit;
        number /= 10;
    }
    n % (sum + product) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_divisibility() {
        assert!(check_divisibility(99));
        assert!(!check_divisibility(32));
        assert!(!check_divisibility(8));
    }
}
