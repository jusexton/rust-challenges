struct Aggregations {
    product: i32,
    sum: i32,
}

pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut agg = Aggregations { product: 1, sum: 0 };
    while n > 0 {
        let digit = n % 10;
        agg.product *= digit;
        agg.sum += digit;
        n /= 10;
    }
    agg.product - agg.sum
}

#[cfg(test)]
mod tests {
    use super::subtract_product_and_sum;

    #[test]
    fn digit_product_is_subtracted_from_digit_sum() {
        assert_eq!(15, subtract_product_and_sum(234));
        assert_eq!(21, subtract_product_and_sum(4421))
    }
}
