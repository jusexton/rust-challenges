pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    (1..=n).map(|n| if n % m != 0 { n } else { -n }).sum()
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
