pub fn replace_digits(mut s: String) -> String {
    let bytes = unsafe { s.as_bytes_mut() };
    for idx in (1..bytes.len()).step_by(2) {
        bytes[idx] = bytes[idx - 1] + bytes[idx] - b'0'
    }
    s
}

#[cfg(test)]
mod tests {
    use super::replace_digits;

    #[test]
    fn replaces_all_digits_with_shifted_characters() {
        assert_eq!("abcdef".to_string(), replace_digits("a1c1e1".to_string()));
        assert_eq!(
            "abbdcfdhe".to_string(),
            replace_digits("a1b2c3d4e".to_string())
        )
    }
}
