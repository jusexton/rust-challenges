fn make_looper(s: &str) -> impl FnMut() -> char + '_ {
    let mut char_looper = s.chars().cycle();
    move || char_looper.next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::make_looper;

    #[test]
    fn test_looper_correctly_loops_over_given_characters() {
        let mut looper = make_looper("abc");
        assert_eq!('a', looper());
        assert_eq!('b', looper());
        assert_eq!('c', looper());
        assert_eq!('a', looper());
    }
}
