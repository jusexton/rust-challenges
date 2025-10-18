pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
    arr.sort_unstable_by_key(|item| (item.count_ones(), *item));
    arr
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::sort_integers_by_the_number_of_1_bits::sort_by_bits;

    #[test_case(&[0, 1, 2, 3, 4, 5, 6, 7, 8], &[0, 1, 2, 4, 8, 3, 5, 6, 7])]
    #[test_case(
        &[1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1],
        &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024])
    ]
    fn test_sort_by_bits(arr: &[i32], expected: &[i32]) {
        let actual = sort_by_bits(arr.to_vec());
        assert_eq!(actual, expected.to_vec())
    }
}
