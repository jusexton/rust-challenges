pub fn cats_and_shelves(start: u8, finish: u8) -> u8 {
    let difference = finish - start;
    difference / 3 + difference % 3
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::cats_and_shelves::cats_and_shelves;

    #[test_case(1, 5, 2)]
    #[test_case(1, 1, 0)]
    #[test_case(2, 5, 1)]
    #[test_case(1, 6, 3)]
    fn should_calculate_least_jumps_to_reach_end(start: u8, finish: u8, expected: u8) {
        let actual = cats_and_shelves(start, finish);
        assert_eq!(actual, expected)
    }
}
