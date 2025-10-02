use std::collections::HashMap;

pub fn get_sneaky_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let freq = numbers.into_iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });
    freq.into_iter().filter(|e| e.1 == 2).map(|e| e.0).collect()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::sneaky_numbers::get_sneaky_numbers;

    #[test]
    fn finds_sneaky_numbers() {
        let mut actual = get_sneaky_numbers(vec![0, 1, 1, 0]);
        actual.sort();
        assert_eq!(vec![0, 1], actual);
    }
}
