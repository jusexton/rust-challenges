use std::collections::HashMap;

pub fn contains_nearby_duplicate(numbers: Vec<i32>, k: i32) -> bool {
    let mut positions = HashMap::with_capacity(numbers.len());
    for (number, idx) in numbers.into_iter().zip(0..) {
        match positions.insert(number, idx) {
            Some(prev_idx) if idx - prev_idx <= k => return true,
            _ => {}
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_when_duplicate_values_are_within_k() {
        assert!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        assert!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1))
    }

    #[test]
    fn identifies_when_duplicate_values_are_not_within_k() {
        assert!(!contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2))
    }
}
