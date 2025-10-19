use std::collections::HashMap;

pub fn min_operations(numbers: Vec<i32>) -> i32 {
    let counts = numbers.into_iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    let mut result = 0;
    for value in counts.into_values() {
        if value == 1 {
            return -1;
        }

        result += value / 3;
        if value % 3 != 0 {
            result += 1
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn returns_minimum_operations_to_empty_array() {
        let numbers = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
        assert_eq!(4, min_operations(numbers))
    }
}
