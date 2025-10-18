use std::collections::HashMap;

pub fn frequency_sort(mut numbers: Vec<i32>) -> Vec<i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for x in numbers.iter() {
        *counter.entry(*x).or_default() += 1;
    }

    numbers.sort_by_key(|n| (counter.get(n).unwrap(), -n));
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_sorted_by_frequency() {
        let numbers = vec![2, 3, 1, 3, 2];
        let actual = frequency_sort(numbers);
        assert_eq!(vec![1, 3, 3, 2, 2], actual)
    }
}
