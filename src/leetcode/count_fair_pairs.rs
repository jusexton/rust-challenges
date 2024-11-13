pub fn count_fair_pairs(mut numbers: Vec<i32>, lower: i32, upper: i32) -> i64 {
    let len = numbers.len();
    numbers.sort_unstable();
    (0..len - 1)
        .map(|i| {
            let hi = numbers[i + 1..].partition_point(|&j| numbers[i] + j <= upper);
            if hi == 0 {
                return 0;
            }
            let lo = numbers[i + 1..].partition_point(|&j| numbers[i] + j < lower);
            hi.saturating_sub(lo) as i64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::count_fair_pairs;

    #[test]
    fn count_number_of_fair_pairs() {
        assert_eq!(6, count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6));
        assert_eq!(1, count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11))
    }
}
