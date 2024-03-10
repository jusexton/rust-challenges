fn minimum_common_value(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] == nums2[j] {
            return nums1[i];
        }

        if nums1[i] > nums2[j] {
            j += 1;
        } else {
            i += 1;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::minimum_common_value::minimum_common_value;

    #[test_case(&[1, 2, 3], &[2, 4], 2)]
    #[test_case(&[2, 3], &[2, 4], 2)]
    #[test_case(&[3, 3, 7, 9, 10], &[2, 4, 5, 10], 10)]
    fn test_minimum_common_value(nums1: &[i32], nums2: &[i32], expected: i32) {
        let actual = minimum_common_value(nums1.to_vec(), nums2.to_vec());
        assert_eq!(actual, expected)
    }
}
