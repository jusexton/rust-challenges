fn feast(beast: &str, dish: &str) -> bool {
    let first_chars_match = beast.chars().next().unwrap() == dish.chars().next().unwrap();
    let last_chars_match = beast.chars().last().unwrap() == dish.chars().last().unwrap();
    first_chars_match && last_chars_match
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::feast::feast;

    #[test_case("great blue heron", "garlic naan", true)]
    #[test_case("chickadee", "chocolate cake", true)]
    #[test_case("rhino", "no", false)]
    fn test_feast(
        beast: &str,
        dish: &str,
        expected: bool,
    ) {
        let actual = feast(beast, dish);
        assert_eq!(expected, actual)
    }
}
