use std::collections::{HashMap, HashSet};

fn unique_occurrences(values: Vec<i32>) -> bool {
    let freq_count = values.iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    let mut seen = HashSet::new();
    freq_count.values().all(|c| seen.insert(c))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::unique_number_of_occurrences::unique_occurrences;

    #[test_case(&[1, 2], false)]
    #[test_case(&[1, 2, 2], true)]
    fn test_unique_occurrences(values: &[i32], expected: bool) {
        let actual = unique_occurrences(values.to_vec());
        assert_eq!(actual, expected);
    }
}
