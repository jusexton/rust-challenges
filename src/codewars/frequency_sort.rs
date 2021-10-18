use std::collections::HashMap;

fn frequency_sort(vec: &[i32]) -> Vec<i32> {
    let counter = vec.iter().fold(HashMap::new(), |mut acc, number| {
        *acc.entry(number).or_insert(0) += 1;
        acc
    });

    let mut sorted = vec.to_vec();
    sorted.sort_by(|a, b| {
        let x = counter.get(a).unwrap();
        let y = counter.get(b).unwrap();
        y.cmp(x).then(a.cmp(b))
    });

    sorted
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::frequency_sort::frequency_sort;

    #[test_case([1, 2, 3, 3, 4, 5, 5, 6, 6], [3, 3, 5, 5, 6, 6, 1, 2, 4])]
    #[test_case([2, 3, 5, 3, 7, 9, 5, 3, 7], [3, 3, 3, 5, 5, 7, 7, 2, 9])]
    fn should_return_vec_sorted_by_element_frequency(input: [i32; 9], expected: [i32; 9]) {
        let actual = frequency_sort(&input);
        assert_eq!(actual, expected)
    }
}
