fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next().unwrap() == dish.chars().next().unwrap()
        && beast.chars().last().unwrap() == dish.chars().last().unwrap()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::feast::feast;

    #[test_case("great blue heron", "garlic naan", true)]
    #[test_case("chickadee", "chocolate cake", true)]
    #[test_case("rhino", "no", false)]
    fn should_return_whether_a_beast_can_bring_a_given_plate(
        beast: &str,
        dish: &str,
        expected: bool,
    ) {
        let actual = feast(beast, dish);
        assert_eq!(expected, actual)
    }
}
