pub fn build_array(numbers: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(numbers.len());
    for &number in numbers.iter() {
        res.push(numbers[number as usize]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_array() {
        assert_eq!(vec![0, 1, 2, 4, 5, 3], build_array(vec![0, 2, 1, 5, 3, 4]))
    }
}
