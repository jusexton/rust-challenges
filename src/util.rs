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
    ($($t:tt),*) => {
        vec![$(vec![$t.0, $t.1]),*]
    };
}
