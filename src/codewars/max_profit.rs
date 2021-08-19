use std::cmp;

fn max_profit(quotes: &[u32]) -> u32 {
    let mut profit = 0;
    let mut max = quotes[quotes.len() - 1];
    for index in (0..quotes.len() - 1).rev() {
        let quote = quotes[index];
        max = cmp::max(max, quote);
        if quote < max {
            profit += max - quote
        }
    }
    profit
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::max_profit::max_profit;

    #[test_case(&[1, 2, 3, 4, 5, 6] , 15)]
    #[test_case(&[6, 5, 4, 3, 2, 1], 0)]
    #[test_case(&[1, 2, 10, 3, 2, 7, 3, 2], 26)]
    fn should_return_max_profit_from_stock_quotes(quotes: &[u32], expected: u32) {
        let actual = max_profit(quotes);
        assert_eq!(actual, expected);
    }
}
