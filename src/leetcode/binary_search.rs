pub fn search(numbers: Vec<i32>, target: i32) -> i32 {
    match numbers.binary_search(&target) {
        Ok(i) => i as i32,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_element_index_when_it_exists() {
        assert_eq!(4, search(vec![-1, 0, 3, 5, 9, 12], 9))
    }

    #[test]
    fn negative_one_when_element_does_not_exist() {
        assert_eq!(-1, search(vec![-1, 0, 3, 5, 9, 12], 20))
    }
}
