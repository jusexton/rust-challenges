use std::collections::HashMap;

pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let tasks = tasks.iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    let mut result = 0;
    for value in tasks.values() {
        if *value == 1 {
            return -1;
        }
        result += (value + 2) / 3;
    }
    result
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::minimum_rounds::minimum_rounds;

    #[test_case(&[1, 1, 2, 2], 2)]
    fn should_produce_minimum_number_of_rounds(tasks: &[i32], expected: i32) {
        let actual = minimum_rounds(tasks.to_vec());
        assert_eq!(actual, expected)
    }
}
