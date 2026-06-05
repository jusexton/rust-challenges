/// Creates a [Vec] of [String] given string literals. Helpful macro to eliminate the boilerplate
/// around creating a String from a literal.
#[macro_export]
macro_rules! string_vec {
    () => {
        Vec::<String>::new()
    };
    ($($str:expr),+ $(,)?) => {
        vec![$($str.to_string()),+]
    };
}

/// Creates a [Vec] of [Vec] given tuples. Helpful macro to eliminate the boilerplate
/// around creating a Vec<Vec<i32>> which is a common way for leetcode problems to
/// structure cell locations in Rust.
#[macro_export]
macro_rules! cell_vec {
    () => {
        Vec::<Vec<i32>>::new()
    };
    ($($t:tt),+ $(,)?) => {
        vec![$(vec![$t.0, $t.1]),+]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn string_vec_empty_produces_empty_vec() {
        let result: Vec<String> = string_vec![];
        assert!(result.is_empty());
    }

    #[test]
    fn string_vec_produces_vec_of_owned_strings() {
        let result = string_vec!["hello", "world"];
        assert_eq!(result, vec!["hello".to_string(), "world".to_string()]);
    }

    #[test]
    fn string_vec_allows_trailing_comma() {
        let result = string_vec!["a", "b", "c",];
        assert_eq!(
            result,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn cell_vec_empty_produces_empty_vec() {
        let result: Vec<Vec<i32>> = cell_vec![];
        assert!(result.is_empty());
    }

    #[test]
    fn cell_vec_produces_vec_of_row_col_pairs() {
        let result = cell_vec![(0, 1), (2, 3)];
        assert_eq!(result, vec![vec![0, 1], vec![2, 3]]);
    }

    #[test]
    fn cell_vec_allows_trailing_comma() {
        let result = cell_vec![(1, 2),];
        assert_eq!(result, vec![vec![1, 2]]);
    }
}
