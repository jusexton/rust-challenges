pub fn product_except_self(numbers: Vec<i32>) -> Vec<i32> {
    let n = numbers.len();
    let mut result = vec![1; n];

    let mut prefix_product = 1;
    for idx in 0..n {
        result[idx] *= prefix_product;
        prefix_product *= numbers[idx];
    }

    let mut suffix_product = 1;
    for idx in (0..n).rev() {
        result[idx] *= suffix_product;
        suffix_product *= numbers[idx];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::product_except_self;

    #[test]
    fn calculate_product_of_all_elements_except_self() {
        let numbers = vec![1, 2, 3, 4];
        let result = product_except_self(numbers);
        assert_eq!(vec![24, 12, 8, 6], result);

        let numbers = vec![-1, 1, 0, -3, 3];
        let result = product_except_self(numbers);
        assert_eq!(vec![0, 0, 9, 0, 0], result)
    }
}
