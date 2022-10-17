use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut compliments = HashMap::new();
    nums.iter()
        .enumerate()
        .find_map(|(i, n)| {
            compliments
                .get(n)
                .and_then(|j| Some(vec![i as i32, *j]))
                .or_else(|| compliments.insert(target - n, i as i32).and(None))
        })
        .unwrap_or(vec![-1, -1])
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::two_sum::two_sum;

    #[test_case(&[2,7,11,15], 9, &[0,1])]
    #[test_case(&[3,2,4], 6, &[1,2])]
    fn test_two_sum(numbers: &[i32], target: i32, expected: &[i32]) {
        let mut result = two_sum(numbers.to_vec(), target);
        // Sort the result because the order of the indexes returned does not matter
        result.sort();
        assert_eq!(result, expected.to_vec());
    }
}
