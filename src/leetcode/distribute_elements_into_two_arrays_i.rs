pub fn result_array(numbers: Vec<i32>) -> Vec<i32> {
    let (mut left, mut right) = (vec![numbers[0]], vec![numbers[1]]);
    for number in numbers.into_iter().skip(2) {
        match left.last().unwrap() > right.last().unwrap() {
            true => left.push(number),
            false => right.push(number),
        }
    }
    [left, right].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_array() {
        assert_eq!(vec![2, 3, 1], result_array(vec![2, 1, 3]));
        assert_eq!(vec![5, 3, 4, 8], result_array(vec![5, 4, 3, 8]));
    }
}
