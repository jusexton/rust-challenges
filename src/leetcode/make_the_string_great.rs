// 1. First solution
// fn make_good(s: String) -> String {
//     let mut result = String::with_capacity(s.len());
//     for char in s.chars() {
//         if !result.is_empty() && (char as u8).abs_diff(result.chars().last().unwrap() as u8) == 32 {
//             result.pop();
//         } else {
//             result.push(char);
//         }
//     }
//     result
// }

fn make_good(s: String) -> String {
    let mut res = String::new();
    for ch in s.into_bytes() {
        match res.chars().last() {
            Some(last) if (last as u8).abs_diff(ch) == 32 => {
                res.pop();
            }
            _ => {
                res.push(ch as char);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::make_the_string_great::make_good;

    #[test_case("leEeetcode", "leetcode")]
    #[test_case("abBAcC", "")]
    #[test_case("s", "s")]
    fn should_produce_length_of_largest_substring_between_duplicates(s: &str, expected: &str) {
        let actual = make_good(s.to_string());
        assert_eq!(actual, expected.to_string());
    }
}
