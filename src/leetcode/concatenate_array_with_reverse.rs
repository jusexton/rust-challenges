pub fn concat_with_reverse(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = numbers.clone();
    result.extend(numbers.into_iter().rev());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_with_reverse() {
        assert_eq!(Vec::<i32>::new(), concat_with_reverse(Vec::new()));
        assert_eq!(vec![1, 1], concat_with_reverse(vec![1]));
        assert_eq!(vec![1, 2, 3, 3, 2, 1], concat_with_reverse(vec![1, 2, 3]));
    }
}
