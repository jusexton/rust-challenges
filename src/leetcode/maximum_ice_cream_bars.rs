pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
    costs.sort_unstable();
    costs
        .into_iter()
        .take_while(|price| {
            coins -= price;
            coins >= 0
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_ice_cream() {
        assert_eq!(max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
        assert_eq!(max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
    }
}
