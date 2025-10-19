use std::cmp;

pub fn max_chunks_to_sorted(numbers: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut max = i32::MIN;
    for (i, number) in numbers.into_iter().enumerate() {
        max = cmp::max(max, number);
        if max == i as i32 {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max_number_of_chunks_to_sort_array() {
        assert_eq!(4, max_chunks_to_sorted(vec![1, 0, 2, 3, 4]))
    }
}
