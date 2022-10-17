fn coin_combo(cents: u64) -> Vec<u64> {
    let qs = cents / 25;
    let ds = (cents - qs * 25) / 10;
    let ns = (cents - qs * 25 - ds * 10) / 5;
    let ps = (cents - qs * 25 - ds * 10) % 5;
    vec![ps, ns, ds, qs]
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::coincombo::coin_combo;

    #[test_case(0, [0, 0, 0, 0])]
    #[test_case(1, [1, 0, 0, 0])]
    #[test_case(5, [0, 1, 0, 0])]
    #[test_case(10, [0, 0, 1, 0])]
    #[test_case(25, [0, 0, 0, 1])]
    #[test_case(100, [0, 0, 0, 4])]
    #[test_case(101, [1, 0, 0, 4])]
    #[test_case(105, [0, 1, 0, 4])]
    #[test_case(110, [0, 0, 1, 4])]
    fn should_return_as_few_coins_as_possible(cents: u64, expected: [u64; 4]) {
        let result = coin_combo(cents);
        assert_eq!(result, expected.to_vec());
    }
}
