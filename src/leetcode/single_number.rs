pub fn single_number(numbers: Vec<i32>) -> i32 {
    numbers.into_iter().fold(0, |acc, curr| acc ^ curr)
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn finds_number_that_appears_once() {
        assert_eq!(2, single_number(vec![1, 1, 2]))
    }
}
