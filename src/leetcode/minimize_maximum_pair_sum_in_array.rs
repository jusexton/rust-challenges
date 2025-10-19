pub fn min_pair_sum(mut numbers: Vec<i32>) -> i32 {
    numbers.sort_unstable();

    let n = numbers.len();
    let mut res = 0;
    for i in 0..n {
        res = res.max(numbers[i] + numbers[n - i - 1])
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimizes_pairs_for_smallest_max_sum() {
        assert_eq!(7, min_pair_sum(vec![3, 5, 2, 3]))
    }
}
