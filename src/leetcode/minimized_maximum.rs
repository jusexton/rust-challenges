pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let (mut left, mut right) = (1, 100_000);
    while left < right {
        let mid = (left + right) / 2;
        let sum = quantities.iter().map(|q| (q + mid - 1) / mid).sum::<i32>();
        match sum > n {
            true => left = mid + 1,
            false => right = mid,
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_minimized_maximum_products_per_store() {
        assert_eq!(3, minimized_maximum(6, vec![11, 6]))
    }
}
