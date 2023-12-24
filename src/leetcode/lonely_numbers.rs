use std::collections::HashMap;

pub fn find_lonely(numbers: Vec<i32>) -> Vec<i32> {
    let freq_map = numbers.into_iter().fold(HashMap::new(), |mut acc, number| {
        *acc.entry(number).or_insert(0) += 1;
        acc
    });

    let mut result = Vec::new();
    for (number, count) in &freq_map {
        if count == &1
            && !freq_map.contains_key(&(number - 1))
            && !freq_map.contains_key(&(number + 1))
        {
            result.push(*number)
        }
    }
    result
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::hash::Hash;

    use test_case::test_case;

    use crate::leetcode::lonely_numbers::find_lonely;

    #[test_case(&[10,6,5,8], &[10, 8])]
    #[test_case(&[1,3,5,3], &[1, 5])]
    fn should_produce_correct_lonely_numbers(input: &[i32], expected: &[i32]) {
        let actual = find_lonely(input.to_vec());

        assert!(eq_ignore_order(actual, expected.to_vec()));
    }

    fn eq_ignore_order<T>(a: Vec<T>, b: Vec<T>) -> bool
    where
        T: Eq + Hash,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }
}
