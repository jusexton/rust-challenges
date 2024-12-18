use std::cmp::Ordering;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, numbers.len() - 1);
    while left < right {
        let sum = numbers[left] + numbers[right];
        match sum.cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
            Ordering::Greater => right -= 1,
        }
    }

    // We assume there is always a solution.
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_sum_indices_are_found() {
        assert_eq!(vec![1, 2], two_sum(vec![2, 7, 11, 15], 9))
    }
}
