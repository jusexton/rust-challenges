pub fn divide_array(mut numbers: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    numbers.sort();
    let mut result = Vec::with_capacity(numbers.len() / 3);
    for chunk in numbers.chunks(3) {
        let vec_chunk = chunk.to_vec();
        match vec_chunk[2] - vec_chunk[0] <= k {
            true => result.push(vec_chunk),
            false => return vec![],
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::divide_array_into_arrays_with_max_difference::divide_array;

    #[test_case(&[1,3,4,8,7,9,3,5,1], 2, &[
        &[1,1,3],
        &[3,4,5],
        &[7,8,9]
    ])]
    #[test_case(&[15,13,12,13,12,14,12,2,3,13,12,14,14,13,5,12,12,2,13,2,2], 2, &[])]
    fn test_divide(numbers: &[i32], k: i32, expected: &[&[i32]]) {
        let actual = divide_array(numbers.to_vec(), k);
        assert_eq!(actual, expected.to_vec())
    }
}
