pub fn results_array(numbers: Vec<i32>, k: i32) -> Vec<i32> {
    numbers
        .windows(k as usize)
        .map(|w| {
            let (mut prev, mut max) = (w[0], w[0]);
            for &val in w.iter().skip(1) {
                if prev + 1 != val {
                    return -1;
                }
                max = max.max(val);
                prev = val;
            }
            max
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::results_array;

    #[test]
    fn non_consecutive_values() {
        assert_eq!(vec![-1, -1], results_array(vec![2, 2, 2, 2, 2], 4))
    }

    #[test]
    fn zipped_consecutive_values() {
        assert_eq!(
            vec![-1, 3, -1, 3, -1],
            results_array(vec![3, 2, 3, 2, 3, 2], 2)
        )
    }

    #[test]
    fn k_equals_n() {
        assert_eq!(vec![3], results_array(vec![2, 3], 2));
        assert_eq!(vec![-1], results_array(vec![3, 2], 2))
    }
}
