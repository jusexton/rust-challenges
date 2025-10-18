use std::cmp;

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut cheapest_buy = i32::MAX;
    let mut max_profit = 0;

    for price in prices {
        cheapest_buy = cmp::min(price, cheapest_buy);
        max_profit = cmp::max(price - cheapest_buy, max_profit)
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::best_time_to_buy_and_sell_stock::max_profit;

    #[test_case(&[], 0)]
    #[test_case(&[4,3,2,1], 0)]
    #[test_case(&[7,1,5,3,6,4], 5)]
    fn should_return_max_profit(numbers: &[i32], expected: i32) {
        let actual = max_profit(numbers.to_vec());
        assert_eq!(expected, actual);
    }
}
