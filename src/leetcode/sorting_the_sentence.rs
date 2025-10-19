pub fn sort_sentence(s: String) -> String {
    let mut words: Vec<_> = s.split_ascii_whitespace().map(String::from).collect();
    words.sort_by_key(|word| word.chars().last().unwrap() as u8);
    words.iter_mut().for_each(|word| {
        word.pop();
    });
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use crate::leetcode::sorting_the_sentence::sort_sentence;

    #[test]
    fn single_word() {
        assert_eq!("hello", sort_sentence("hello1".to_string()))
    }

    #[test]
    fn two_words() {
        assert_eq!("hello world", sort_sentence("world2 hello1".to_string()))
    }
}
