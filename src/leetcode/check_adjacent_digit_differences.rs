pub fn is_adjacent_diff_at_most_two(s: String) -> bool {
    s.as_bytes().windows(2).all(|w| w[0].abs_diff(w[1]) <= 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_adjacent_diff_at_most_two() {
        assert!(is_adjacent_diff_at_most_two("132".to_string()));
        assert!(!is_adjacent_diff_at_most_two("129".to_string()));
    }
}
