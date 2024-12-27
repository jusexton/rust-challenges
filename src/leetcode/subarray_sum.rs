pub fn min_sub_array_len(target: i32, numbers: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, 0);
    let mut sum = 0;
    let mut min_len = usize::MAX;
    while right < numbers.len() {
        sum += numbers[right];
        right += 1;

        while sum >= target {
            min_len = std::cmp::min(min_len, right - left);
            sum -= numbers[left];
            left += 1;
        }
    }

    match min_len == usize::MAX {
        true => 0,
        false => min_len as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_length_of_smallest_subarr_that_sums_to_target() {
        assert_eq!(2, min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]))
    }
}
