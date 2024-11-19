use std::collections::HashMap;

pub fn maximum_subarray_sum(numbers: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut seen = HashMap::with_capacity(k);
    let mut sum = 0i64;
    for &val in &numbers[0..k] {
        *seen.entry(val as i64).or_insert(0) += 1;
        sum += val as i64;
    }

    let mut res = if seen.len() == k { sum } else { 0 };
    for i in k..numbers.len() {
        let (prev, current) = (numbers[i - k] as i64, numbers[i] as i64);
        seen.entry(prev).and_modify(|e| *e -= 1);
        *seen.entry(current).or_insert(0) += 1;
        if let Some(0) = seen.get(&prev) {
            seen.remove(&prev);
        }

        sum -= prev;
        sum += current;
        if seen.len() == k {
            res = res.max(sum);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::maximum_subarray_sum;

    #[test]
    fn finds_max_subarray_sum() {
        assert_eq!(12, maximum_subarray_sum(vec![9, 9, 9, 1, 2, 3], 3));
        assert_eq!(15, maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3))
    }

    #[test]
    fn no_valid_windows() {
        assert_eq!(0, maximum_subarray_sum(vec![4, 4, 4], 3))
    }
}
