pub fn make_fancy_string(s: String) -> String {
    s.as_bytes()
        .chunk_by(|a, b| a == b)
        .flat_map(|chunk| chunk.iter().take(2).map(|c| *c as char))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::delete_characters_to_make_fancy_string::make_fancy_string;

    #[test]
    fn makes_fancy_string() {
        let result = make_fancy_string("leeetcode".to_string());
        assert_eq!("leetcode".to_string(), result)
    }
}
