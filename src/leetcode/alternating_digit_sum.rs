pub fn alternate_digit_sum(mut number: i32) -> i32 {
    let mut result = 0;
    let mut sign = 1;
    while number > 0 {
        sign *= -1;
        result += (number % 10) * sign;
        number /= 10;
    }
    result * sign
}

#[cfg(test)]
mod tests {
    use super::alternate_digit_sum;

    #[test]
    fn digits_are_summed_with_alternating_signs() {
        let sum = alternate_digit_sum(12345);
        assert_eq!(3, sum);

        let sum = alternate_digit_sum(10);
        assert_eq!(1, sum)
    }
}
