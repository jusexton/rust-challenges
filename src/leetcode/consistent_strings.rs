use std::collections::HashSet;
use std::iter::FromIterator;

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let allowed: HashSet<char> = HashSet::from_iter(allowed.chars());
    words
        .iter()
        .filter(|word| {
            word.chars().all(|c| allowed.contains(&c))
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::leetcode::consistent_strings::count_consistent_strings;

    #[test_case("ab", &["ad","bd","aaab","baa","badab"], 2)]
    #[test_case("abc", &["a","b","c","ab","ac","bc","abc"], 7)]
    fn should_return_the_correct_number_of_consistent_strings(
        allowed: &str,
        words: &[&str],
        expected: i32,
    ) {
        let words = to_string_vec(words);
        let actual = count_consistent_strings(allowed.to_string(), words);
        assert_eq!(expected, actual)
    }

    fn to_string_vec(slices: &[&str]) -> Vec<String> {
        slices.iter().map(|it| String::from(*it)).collect()
    }
}
