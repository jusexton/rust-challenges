fn beeramid(bonus: i32, price: f32) -> usize {
    let mut bonus = bonus as f32;
    let mut level = 0;
    let mut cans = 1;
    while cans as f32 * price <= bonus {
        bonus -= cans as f32 * price;
        cans = cans + 3 + (2 * level);
        level += 1;
    }
    level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_levels() {
        assert_eq!(0, beeramid(10, 12.5));
        assert_eq!(1, beeramid(15, 12.5));
        assert_eq!(2, beeramid(10, 2.0));
        assert_eq!(2, beeramid(11, 2.0));
        assert_eq!(12, beeramid(1500, 2.0));
    }
}
