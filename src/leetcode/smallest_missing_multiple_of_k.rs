pub fn missing_multiple(numbers: Vec<i32>, k: i32) -> i32 {
    for number in (k..).step_by(k as usize) {
        if !numbers.contains(&number) {
            return number;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_multiple() {
        assert_eq!(5, missing_multiple(vec![1, 4, 7, 10, 15], 5));
        assert_eq!(10, missing_multiple(vec![8, 2, 3, 4, 6], 2));
    }
}
