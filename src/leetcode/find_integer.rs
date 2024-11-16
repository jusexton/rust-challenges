pub fn added_integer(a: Vec<i32>, b: Vec<i32>) -> i32 {
    b.iter().min().unwrap() - a.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::added_integer;

    #[test]
    fn finds_the_value_that_was_added_to_initial_array() {
        assert_eq!(5, added_integer(vec![1, 2], vec![6, 7]));
        assert_eq!(-5, added_integer(vec![10], vec![5]))
    }
}
