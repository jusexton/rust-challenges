pub fn find_the_difference(s: String, t: String) -> char {
    fn sum_bytes(s: &str) -> u32 {
        s.bytes().fold(0, |acc, curr| acc + curr as u32)
    }

    char::from_u32(sum_bytes(&t) - sum_bytes(&s)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::find_the_difference;

    #[test]
    fn finds_the_difference_between_two_strings() {
        assert_eq!('y', find_the_difference("".to_string(), "y".to_string()));
        assert_eq!(
            'd',
            find_the_difference("abc".to_string(), "dabc".to_string())
        )
    }
}
