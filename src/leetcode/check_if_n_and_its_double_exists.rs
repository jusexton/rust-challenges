use std::collections::HashSet;

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    let mut seen = HashSet::with_capacity(arr.len());
    for i in arr.iter() {
        if seen.contains(&(i * 2)) || i % 2 == 0 && seen.contains(&(i / 2)) {
            return true;
        }
        seen.insert(i);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::check_if_exist;

    #[test]
    fn result_true_when_double_exists() {
        assert!(check_if_exist(vec![10, 2, 5, 3]))
    }

    #[test]
    fn result_false_when_double_does_not_exist() {
        assert!(!check_if_exist(vec![3, 1, 7, 11]))
    }
}
