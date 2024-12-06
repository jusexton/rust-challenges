use std::collections::HashSet;

pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    let banned: HashSet<_> = banned.into_iter().collect();
    (1..=n)
        .filter(|value| !banned.contains(value))
        .scan(0, |sum, value| match *sum + value <= max_sum {
            true => {
                *sum += value;
                Some(value)
            }
            false => None,
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::max_count;

    #[test]
    fn finds_max_count_of_numbers_in_range() {
        assert_eq!(2, max_count(vec![1, 6, 5], 5, 6));
        assert_eq!(0, max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1));
        assert_eq!(7, max_count(vec![11], 7, 50))
    }
}
