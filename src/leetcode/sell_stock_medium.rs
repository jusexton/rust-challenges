fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .windows(2)
        .filter(|window| window[0] < window[1])
        .map(|window| window[1] - window[0])
        .sum()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::sell_stock_medium::max_profit;

    #[test_case(&[], 0)]
    #[test_case(&[4,3,2,1], 0)]
    #[test_case(&[7,1,5,3,6,4], 7)]
    fn should_return_max_profit(numbers: &[i32], expected: i32) {
        let actual = max_profit(numbers.to_vec());
        assert_eq!(expected, actual);
    }
}
