use std::collections::HashMap;

pub fn intersect(numbers_one: Vec<i32>, numbers_two: Vec<i32>) -> Vec<i32> {
    let mut counts = numbers_one
        .into_iter()
        .fold(HashMap::new(), |mut acc, curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        });

    let mut result = Vec::new();
    for number in numbers_two {
        if let Some(count) = counts.get_mut(&number) {
            if *count > 0 {
                result.push(number);
                *count -= 1;
            }
        }
    }
    result
}
g
#[cfg(test)]
mod tests {
    use std::vec;

    use super::intersect;

    #[test]
    fn test_intersect() {
        let result = intersect(vec![1, 2, 3, 3, 1, 5, 6], vec![1, 3, 3, 1]);
        let expected = vec![1, 3, 3, 1];
        assert_eq!(expected, result)
    }
}
