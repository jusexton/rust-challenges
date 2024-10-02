use std::collections::HashMap;

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut numbers = arr.clone();
    numbers.sort_unstable();

    let mut ranks = HashMap::new();
    for number in numbers.iter() {
        let n = ranks.len();
        ranks.entry(number).or_insert(n + 1);
    }

    let mut result = vec![0; numbers.len()];
    for idx in 0..arr.len() {
        result[idx] = *ranks.get(&arr[idx]).unwrap() as i32;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::array_rank_transform;

    #[test]
    fn test() {
        assert_eq!(vec![4, 1, 2, 3], array_rank_transform(vec![40, 10, 20, 30]));
        assert_eq!(vec![1, 1, 1], array_rank_transform(vec![100, 100, 100]));
        assert_eq!(
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3],
            array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12])
        );
    }
}
