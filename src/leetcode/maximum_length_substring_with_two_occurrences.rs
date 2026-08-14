use std::collections::HashMap;

pub fn maximum_length_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut count: HashMap<u8, i32> = HashMap::new();
    let mut left = 0;
    let mut best = 0;

    for right in 0..bytes.len() {
        *count.entry(bytes[right]).or_insert(0) += 1;

        while count[&bytes[right]] > 2 {
            *count.entry(bytes[left]).or_insert(0) -= 1;
            left += 1;
        }

        best = best.max(right - left + 1);
    }

    best as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_length_substring() {
        assert_eq!(4, maximum_length_substring("bcbbbcba".to_string()));
        assert_eq!(2, maximum_length_substring("aaaa".to_string()));
    }
}
