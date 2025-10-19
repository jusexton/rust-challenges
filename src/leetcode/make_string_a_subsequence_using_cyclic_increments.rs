pub fn can_make_subsequence(source: String, target: String) -> bool {
    let target_len = target.len();
    let target_bytes = target.as_bytes();
    let mut target_idx = 0;
    for ch in source.bytes() {
        if target_idx < target_len && (26 + target_bytes[target_idx] - ch) % 26 <= 1 {
            target_idx += 1;
        }
    }
    target_idx == target_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_when_subsequences_can_be_made() {
        assert!(can_make_subsequence("abc".to_string(), "ad".to_string()));
        assert!(can_make_subsequence("zc".to_string(), "ad".to_string()))
    }

    #[test]
    fn determines_when_subsequences_can_not_be_made() {
        assert!(!can_make_subsequence("ab".to_string(), "d".to_string()))
    }
}
