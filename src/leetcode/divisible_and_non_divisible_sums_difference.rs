pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let num1 = (1..=n)
        .filter(|&n| !(n as u32).is_multiple_of(m as u32))
        .sum::<i32>();
    let num2 = (1..=n)
        .filter(|&n| (n as u32).is_multiple_of(m as u32))
        .sum::<i32>();
    num1 - num2
}

#[cfg(test)]
mod tests {
    use super::difference_of_sums;
    use test_case::test_case;

    #[test_case(5, 6, 15)]
    #[test_case(5, 1, -15)]
    fn test_difference_of_sums(n: i32, m: i32, expected: i32) {
        assert_eq!(difference_of_sums(n, m), expected);
    }
}
