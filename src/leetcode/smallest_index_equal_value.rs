pub fn smallest_equal(numbers: Vec<i32>) -> i32 {
    for (i, number) in numbers.into_iter().enumerate() {
        if i % 10 == number as usize {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::leetcode::smallest_index_equal_value::smallest_equal;

    #[test]
    fn finds_smallest_index_with_equal_value() {
        assert_eq!(0, smallest_equal(vec![0, 1, 2]));
    }

    #[test]
    fn negative_one_when_no_valid_value() {
        assert_eq!(-1, smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
    }
}
