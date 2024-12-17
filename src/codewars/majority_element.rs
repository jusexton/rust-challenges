use std::collections::HashMap;

pub fn majority_element(numbers: Vec<i32>) -> i32 {
    let freq = numbers.into_iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });
    freq.into_iter().max_by_key(|e| e.1).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_majority_element() {
        assert_eq!(2, majority_element(vec![1, 2, 2, 1, 2]))
    }
}
