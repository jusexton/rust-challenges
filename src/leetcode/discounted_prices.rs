pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    for i in 0..prices.len() {
        while let Some(&idx) = stack.last() {
            if prices[idx] < prices[i] {
                break;
            }
            prices[stack.pop().unwrap()] -= prices[i];
        }
        stack.push(i);
    }
    prices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_final_prices_after_discounts() {
        assert_eq!(vec![4, 2, 4, 2, 3], final_prices(vec![8, 4, 6, 2, 3]))
    }
}
