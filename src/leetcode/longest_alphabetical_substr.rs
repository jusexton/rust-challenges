pub fn longest_continuous_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let (_, max_len) = (0..s.len())
        .map(|i| i != 0 && (bytes[i] == bytes[i - 1] + 1))
        .fold((0, 0), |(cur_len, max_len), inc| {
            let new_curr = if inc { cur_len + 1 } else { 1 };
            let new_max = max_len.max(new_curr);
            (new_curr, new_max)
        });
    max_len
}

#[cfg(test)]
mod tests {
    use crate::leetcode::longest_alphabetical_substr::longest_continuous_substring;

    #[test]
    fn single_char() {
        assert_eq!(1, longest_continuous_substring("a".to_string()))
    }

    #[test]
    fn single_alphabetical_substrings_exists() {
        assert_eq!(5, longest_continuous_substring("abcde".to_string()))
    }

    #[test]
    fn multiple_alphabetical_substrings_exist() {
        assert_eq!(2, longest_continuous_substring("abacaba".to_string()))
    }
}
