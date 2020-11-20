fn sort_numbers(vector: &Vec<i32>) -> Vec<i32> {
    let mut test = vector.clone();
    test.sort();
    return test;
}

#[cfg(test)]
mod tests {
    use crate::codewars::sort_numbers::sort_numbers;

    #[test]
    fn should_return_empty_vector() {
        let input: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, sort_numbers(&input))
    }

    #[test]
    fn should_return_sorted_vector() {
        let input = vec![1, 4, 3, 2];
        let expected = vec![1, 2, 3, 4];
        assert_eq!(expected, sort_numbers(&input))
    }
}