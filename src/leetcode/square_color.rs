pub fn square_is_white(coordinates: String) -> bool {
    let bytes = coordinates.as_bytes();
    (bytes[0] - 97 + bytes[1] - 48).is_multiple_of(2)
}

#[cfg(test)]
mod tests {
    use crate::leetcode::square_color::square_is_white;

    #[test]
    fn white_square() {
        assert!(square_is_white("a2".to_string()));
        assert!(square_is_white("d7".to_string()));
    }

    #[test]
    fn black_square() {
        assert!(!square_is_white("a1".to_string()));
        assert!(!square_is_white("e5".to_string()));
    }
}
