pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut characters = [0i32; 26];
    magazine.bytes().for_each(|b| {
        characters[b as usize - 97] += 1;
    });
    ransom_note.bytes().for_each(|b| {
        characters[b as usize - 97] -= 1;
    });
    characters.iter().all(|&x| x >= 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_if_ransom_note_can_be_constructed_from_magazine() {
        assert!(can_construct("aa".to_string(), "aab".to_string()))
    }
}
