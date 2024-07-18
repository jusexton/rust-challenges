pub fn remove_duplicates(numbers: &mut [i32]) -> i32 {
    if numbers.is_empty() {
        return 0;
    }

    let mut next_slot = 0;
    for curr_idx in 1..numbers.len() {
        if numbers[next_slot] != numbers[curr_idx] {
            next_slot += 1;
            numbers[next_slot] = numbers[curr_idx];
        }
    }

    (next_slot + 1) as i32
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn test_remove_duplicate(expected: &[i32], numbers: &mut [i32]) {
        let k = remove_duplicates(numbers) as usize;
        assert_eq!(expected.len(), k);
        for idx in 0..k {
            assert_eq!(expected[idx], numbers[idx])
        }
    }

    #[test]
    fn duplicates_are_removed() {
        let mut numbers = vec![1, 1, 2];
        let expected = vec![1, 2];
        test_remove_duplicate(&expected, &mut numbers);
    }
}
