pub fn rearrange(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; numbers.len()];
    let mut pos_index = 0;
    let mut neg_index = 1;
    for number in numbers {
        if number > 0 {
            result[pos_index] = number;
            pos_index += 2;
        } else {
            result[neg_index] = number;
            neg_index += 2;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::leetcode::rearrange_array_elements_by_sign::rearrange;

    #[test_case(&[-1, 1], &[1, -1])]
    #[test_case(&[3,1,-2,-5,2,-4], &[3,-2,1,-5,2,-4])]
    fn should_correctly_rearrange_array(input: &[i32], expected: &[i32]) {
        let actual = rearrange(input.to_vec());

        assert_eq!(actual, expected.to_vec())
    }
}
