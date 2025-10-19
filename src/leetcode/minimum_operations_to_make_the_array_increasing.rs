pub fn min_operations(mut numbers: Vec<i32>) -> i32 {
    let mut operation_count = 0;
    for i in 1..numbers.len() {
        if numbers[i] <= numbers[i - 1] {
            let diff = numbers[i - 1] - numbers[i] + 1;
            operation_count += diff;
            numbers[i] += diff;
        }
    }
    operation_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_minimum_operations_to_make_array_increasing() {
        assert_eq!(14, min_operations(vec![1, 5, 2, 4, 1]));
        assert_eq!(3, min_operations(vec![1, 1, 1]))
    }
}
