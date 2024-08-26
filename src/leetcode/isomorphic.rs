pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut map_one = [0; 256];
    let mut map_two = [0; 256];
    for idx in 0..s.len() {
        let (c1, c2) = (s.as_bytes()[idx] as usize, t.as_bytes()[idx] as usize);
        if map_one[c1] != map_two[c2] {
            return false;
        }

        map_one[c1] = idx + 1;
        map_two[c2] = idx + 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_isomorphic;

    #[test]
    fn isomorphic() {
        assert!(is_isomorphic("egg".to_string(), "add".to_string()));
    }

    #[test]
    fn not_isomorphic() {
        assert!(!is_isomorphic("foo".to_string(), "bar".to_string()));
        assert!(!is_isomorphic("badc".to_string(), "baba".to_string()));
    }
}
