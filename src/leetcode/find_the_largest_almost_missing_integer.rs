pub fn largest_integer(numbers: Vec<i32>, k: i32) -> i32 {
    let mut counts = [0; 51];
    for number in numbers.iter() {
        counts[*number as usize] += 1;
    }

    let n = numbers.len();
    let mut result = -1;
    for (idx, number) in numbers.into_iter().enumerate() {
        if k as usize == n || (counts[number as usize] == 1 && (k == 1 || idx == 0 || idx == n - 1))
        {
            result = result.max(number)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_integer() {
        assert_eq!(7, largest_integer(vec![3, 9, 2, 1, 7], 3));
        assert_eq!(3, largest_integer(vec![3, 9, 7, 2, 1, 7], 4));
    }
}
