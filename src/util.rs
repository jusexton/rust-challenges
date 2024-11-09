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
