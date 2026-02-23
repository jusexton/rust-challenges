use std::collections::HashSet;

pub fn has_all_codes(s: String, k: i32) -> bool {
    let k = k as usize;
    let n = s.len();
    let expected_count = 2_usize.pow(k as u32);

    if n < k || (n - k + 1) < expected_count {
        return false;
    }

    let mut seen = HashSet::new();
    for window in s.as_bytes().windows(k) {
        seen.insert(window);
        if seen.len() == expected_count {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::check_if_a_string_contains_all_binary_codes_of_size_k::has_all_codes;

    #[test_case("0110", 1, true)]
    #[test_case("0110", 2, false)]
    #[test_case("00110110", 2, true)]
    fn test_has_all_codes(s: &str, k: i32, expected: bool) {
        assert_eq!(expected, has_all_codes(s.to_string(), k))
    }
}
