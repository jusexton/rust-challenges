use std::collections::VecDeque;

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = VecDeque::with_capacity(digits.len() + 1);
    let mut carry = 1;
    for d in digits.iter().rev() {
        if carry + d == 10 {
            result.push_front(0)
        } else {
            result.push_front(carry + d);
            carry = 0
        }
    }

    if carry == 1 {
        result.push_front(1)
    }

    result.into()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::leetcode::plus_one::plus_one;

    #[test_case(&[0], &[1])]
    #[test_case(&[1, 2, 3], &[1, 2, 4])]
    #[test_case(&[9, 9], &[1, 0, 0])]
    #[test_case(&[1, 2, 9, 9], &[1, 3, 0, 0])]
    #[test_case(&[9, 9, 1], &[9, 9, 2])]
    fn should_correctly_add_one(input: &[i32], expected: &[i32]) {
        let actual = plus_one(input.to_vec());

        assert_eq!(actual.to_vec(), expected)
    }
}
