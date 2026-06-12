pub fn number_of_matches(n: i32) -> i32 {
    n - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_matches() {
        assert_eq!(0, number_of_matches(1));
        assert_eq!(6, number_of_matches(7));
        assert_eq!(13, number_of_matches(14));
        assert_eq!(20, number_of_matches(21));
    }
}
