use std::collections::HashMap;

pub fn find_pairs(numbers: Vec<i32>, k: i32) -> i32 {
    let freq_map = numbers.iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    let pair_count = if k == 0 {
        freq_map.into_values().filter(|&v| v >= 2).count()
    } else {
        freq_map
            .keys()
            .filter(|&&key| freq_map.contains_key(&(key + k)))
            .count()
    };
    pair_count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_unique_pairs() {
        assert_eq!(2, find_pairs(vec![3, 1, 4, 1, 5], 2))
    }

    #[test]
    fn single_pair_when_single_unique_digit() {
        assert_eq!(1, find_pairs(vec![1, 1, 1, 1, 1], 0))
    }

    #[test]
    fn k_zero() {
        assert_eq!(1, find_pairs(vec![1, 3, 1, 5, 4], 0))
    }
}
