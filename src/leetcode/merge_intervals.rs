pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable_by_key(|inter| inter[0]);

    let mut result: Vec<Vec<_>> = vec![intervals[0].to_vec()];
    for curr in intervals.into_iter().skip(1) {
        let last = result.last_mut().unwrap();
        if curr[0] <= last[1] {
            last[1] = last[1].max(curr[1]);
        } else {
            result.push(curr);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn merges_overlapping_time_interavls() {
        assert_eq!(
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        )
    }
}
