fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.into_iter().map(|s| *s as u8).sum()
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::sheep::count_sheep;

    #[test_case(&[], 0)]
    #[test_case(&[false], 0)]
    #[test_case(&[true], 1)]
    #[test_case(&[true, false, true, false], 2)]
    fn should_return_the_number_of_existing_sheep(sheep: &[bool], expected: u8) {
        let sheep_count = count_sheep(sheep);
        assert_eq!(sheep_count, expected);
    }
}
